// Hermetic retry-policy tests for the generated AirtableClient (epic
// follow-up myairtable-rxht, T1).
//
// NO NETWORK. A custom URLProtocol subclass (`StubProtocol`) is installed on a
// dedicated URLSessionConfiguration that we inject into AirtableClient via its
// `session:` parameter. The stub returns a SCRIPTED sequence of HTTP responses
// per request URL+method and counts how many times the client actually hit the
// transport. That attempt count is what proves the retry policy.
//
// Policy under test (must stay aligned with the other targets):
//   - 429            -> ALWAYS retried (then surfaces .rateLimited after maxRetries)
//   - 5xx            -> retried ONLY IF the op is idempotent
//   - create (POST)  -> NOT idempotent: a 5xx is NOT retried (exactly 1 attempt)
//
// We drive the policy through the client's public endpoint methods:
//   - getRecord    -> send(idempotent: true)
//   - createRecords-> send(idempotent: false)
// and keep `baseRetryDelay` tiny so the (jittered) backoff sleeps are negligible.

import Foundation
import Testing

@testable import MyAirtable

#if canImport(FoundationNetworking)
    import FoundationNetworking
#endif

// MARK: - Scripted stub transport

/// A single scripted reply. `body` defaults to a minimal JSON object so success
/// payloads decode cleanly through the client's record envelopes.
private struct StubReply {
    let status: Int
    let headers: [String: String]
    let body: Data

    init(status: Int, headers: [String: String] = [:], body: Data = Data("{}".utf8)) {
        self.status = status
        self.headers = headers
        self.body = body
    }
}

/// Shared, test-controlled state for `StubProtocol`. URLProtocol instances are
/// created by the loading system and can't be configured directly, so the
/// script + attempt counter live in a global guarded by a lock.
private final class StubState: @unchecked Sendable {
    static let shared = StubState()

    private let lock = NSLock()
    /// Queue of replies to hand out, in order, across ALL intercepted requests.
    private var script: [StubReply] = []
    /// Number of requests that reached the transport (i.e. attempts).
    private(set) var attempts = 0

    func reset(script: [StubReply]) {
        lock.lock()
        defer { lock.unlock() }
        self.script = script
        self.attempts = 0
    }

    /// Record one attempt and return the next scripted reply. If the script is
    /// exhausted, the LAST reply is repeated (so "persistent 5xx" scripts can be
    /// short).
    func next() -> StubReply {
        lock.lock()
        defer { lock.unlock() }
        attempts += 1
        if script.count > 1 {
            return script.removeFirst()
        }
        return script.first ?? StubReply(status: 500)
    }

    var attemptCount: Int {
        lock.lock()
        defer { lock.unlock() }
        return attempts
    }
}

/// URLProtocol that intercepts every request and replies from `StubState`.
private final class StubProtocol: URLProtocol {
    override class func canInit(with request: URLRequest) -> Bool { true }
    override class func canonicalRequest(for request: URLRequest) -> URLRequest { request }

    override func startLoading() {
        let reply = StubState.shared.next()
        let url = request.url ?? URL(string: "https://stub.invalid")!
        let response = HTTPURLResponse(
            url: url,
            statusCode: reply.status,
            httpVersion: "HTTP/1.1",
            headerFields: reply.headers
        )!
        client?.urlProtocol(self, didReceive: response, cacheStoragePolicy: .notAllowed)
        client?.urlProtocol(self, didLoad: reply.body)
        client?.urlProtocolDidFinishLoading(self)
    }

    override func stopLoading() {}
}

// MARK: - Helpers

/// Build an AirtableClient whose transport is the stub protocol, with a tiny
/// retry delay so backoff sleeps stay negligible. maxRetries is small + explicit
/// so the "attempts == maxRetries + 1" assertions are exact.
private func makeStubbedClient(maxRetries: Int = 3) -> AirtableClient {
    let config = URLSessionConfiguration.ephemeral
    config.protocolClasses = [StubProtocol.self]
    let session = URLSession(configuration: config)
    return AirtableClient(
        baseId: "appSTUB000000000",
        apiKey: "patSTUB.deadbeefdeadbeefdeadbeefdeadbeef",
        session: session,
        maxRetries: maxRetries,
        baseRetryDelay: 0.001  // ~<=1ms (jittered down from here): hermetic + fast
    )
}

private func isAirtableError(_ error: Error) -> Bool {
    error is AirtableError
}

// MARK: - Suite

@Suite("Retry policy (hermetic stubbed transport)", .serialized)
struct TestRetryPolicy {

    // MARK: 1. Idempotent GET: 503, 503, 200 -> 3 attempts, success.

    @Test("idempotent get retries transient 5xx then succeeds (3 attempts)")
    func idempotentGetRetriesThenSucceeds() async throws {
        StubState.shared.reset(script: [
            StubReply(status: 503),
            StubReply(status: 503),
            StubReply(status: 200, body: Data(#"{"id":"recX","fields":{}}"#.utf8)),
        ])
        let client = makeStubbedClient(maxRetries: 3)

        let data = try await client.getRecord(tableId: "tblSTUB", recordId: "recX")

        #expect(StubState.shared.attemptCount == 3)
        #expect(!data.isEmpty)
    }

    // MARK: 2. Idempotent op, persistent 503 -> maxRetries + 1 attempts then throws.

    @Test("idempotent get on persistent 5xx exhausts retries then throws")
    func idempotentGetExhaustsRetriesThenThrows() async throws {
        StubState.shared.reset(script: [StubReply(status: 503)])  // repeated forever
        let maxRetries = 3
        let client = makeStubbedClient(maxRetries: maxRetries)

        await #expect(throws: (any Error).self) {
            _ = try await client.getRecord(tableId: "tblSTUB", recordId: "recX")
        }
        // initial attempt + maxRetries retries
        #expect(StubState.shared.attemptCount == maxRetries + 1)
    }

    // MARK: 3. NON-idempotent create on 503 -> exactly 1 attempt, throws (no retry).

    @Test("non-idempotent create does NOT retry a 5xx (exactly 1 attempt)")
    func nonIdempotentCreateDoesNotRetry() async throws {
        StubState.shared.reset(script: [StubReply(status: 503)])
        let client = makeStubbedClient(maxRetries: 3)

        let body = Data(#"{"records":[{"fields":{}}]}"#.utf8)
        await #expect(throws: (any Error).self) {
            _ = try await client.createRecords(tableId: "tblSTUB", body: body)
        }
        #expect(StubState.shared.attemptCount == 1)
    }

    // MARK: 4. 429 then 200 -> retried (even though create is non-idempotent).

    @Test("429 is always retried then succeeds")
    func rateLimitIsRetried() async throws {
        StubState.shared.reset(script: [
            StubReply(status: 429, headers: ["Retry-After": "0"]),
            StubReply(status: 200, body: Data(#"{"records":[]}"#.utf8)),
        ])
        // Use a NON-idempotent create to prove 429 retries regardless of idempotency.
        let client = makeStubbedClient(maxRetries: 3)

        let body = Data(#"{"records":[{"fields":{}}]}"#.utf8)
        let data = try await client.createRecords(tableId: "tblSTUB", body: body)

        #expect(StubState.shared.attemptCount == 2)
        #expect(!data.isEmpty)
    }

    // MARK: 5. Persistent 429 surfaces .rateLimited after exhausting retries.

    @Test("persistent 429 surfaces .rateLimited after maxRetries")
    func persistentRateLimitSurfacesRateLimited() async throws {
        StubState.shared.reset(script: [StubReply(status: 429, headers: ["Retry-After": "0"])])
        let maxRetries = 2
        let client = makeStubbedClient(maxRetries: maxRetries)

        do {
            _ = try await client.getRecord(tableId: "tblSTUB", recordId: "recX")
            Issue.record("Expected persistent 429 to throw")
        } catch let error as AirtableError {
            guard case .rateLimited = error else {
                Issue.record("Expected .rateLimited, got \(error)")
                return
            }
        }
        #expect(StubState.shared.attemptCount == maxRetries + 1)
    }
}
