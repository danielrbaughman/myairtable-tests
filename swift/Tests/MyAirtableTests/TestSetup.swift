// Shared fixtures for Swift integration tests.
//
// Walks up from CWD looking for a .env file, parses simple KEY=VALUE lines
// (ignoring blank lines and #-comments), and surfaces AIRTABLE_API_KEY +
// AIRTABLE_BASE_ID as helpers. Mirrors what Rust's `dotenvy::from_filename`
// and Python's conftest.py load_dotenv() do for the other language suites.

import Foundation

@testable import MyAirtable

enum TestSetup {
    /// Construct an Airtable actor wrapper from .env. Fails the test if the
    /// required variables aren't present.
    static func makeAirtable() -> Airtable {
        loadDotEnvIfNeeded()
        let env = ProcessInfo.processInfo.environment
        guard let apiKey = env["AIRTABLE_API_KEY"], !apiKey.isEmpty else {
            fatalError("AIRTABLE_API_KEY not set. Ensure .env is present in the test repo root.")
        }
        guard let baseId = env["AIRTABLE_BASE_ID"], !baseId.isEmpty else {
            fatalError("AIRTABLE_BASE_ID not set. Ensure .env is present in the test repo root.")
        }
        return Airtable(baseId: baseId, apiKey: apiKey)
    }

    /// Construct an Airtable actor wrapper pointed at the real base but with a
    /// bogus API key, so auth-failure (401) tests exercise the key rather than
    /// the base (which would 404). Mirrors C#'s `MakeAirtableWithBadKey`.
    static func makeAirtableWithBadKey() -> Airtable {
        loadDotEnvIfNeeded()
        let env = ProcessInfo.processInfo.environment
        guard let baseId = env["AIRTABLE_BASE_ID"], !baseId.isEmpty else {
            fatalError("AIRTABLE_BASE_ID not set. Ensure .env is present in the test repo root.")
        }
        return Airtable(baseId: baseId, apiKey: "patBOGUS00000.deadbeefdeadbeefdeadbeefdeadbeef")
    }

    /// Load ./env (or any ancestor's .env) into the current process's
    /// environment. Idempotent.
    static func loadDotEnvIfNeeded() {
        if ProcessInfo.processInfo.environment["AIRTABLE_API_KEY"] != nil {
            return
        }
        guard let path = findDotEnv() else { return }
        guard let contents = try? String(contentsOfFile: path, encoding: .utf8) else { return }
        for rawLine in contents.split(separator: "\n", omittingEmptySubsequences: true) {
            let line = rawLine.trimmingCharacters(in: .whitespaces)
            if line.isEmpty || line.hasPrefix("#") { continue }
            guard let eq = line.firstIndex(of: "=") else { continue }
            let key = String(line[..<eq]).trimmingCharacters(in: .whitespaces)
            var value = String(line[line.index(after: eq)...]).trimmingCharacters(in: .whitespaces)
            // Strip matching surrounding quotes.
            if value.count >= 2,
                (value.hasPrefix("\"") && value.hasSuffix("\""))
                    || (value.hasPrefix("'") && value.hasSuffix("'"))
            {
                value = String(value.dropFirst().dropLast())
            }
            setenv(key, value, 1)
        }
    }

    /// Walk from CWD up to the filesystem root looking for ".env".
    private static func findDotEnv() -> String? {
        let fm = FileManager.default
        var dir = URL(fileURLWithPath: fm.currentDirectoryPath)
        while true {
            let candidate = dir.appendingPathComponent(".env")
            if fm.fileExists(atPath: candidate.path) {
                return candidate.path
            }
            let parent = dir.deletingLastPathComponent()
            if parent == dir { return nil }
            dir = parent
        }
    }

    /// Unique primary key for a given test to avoid cross-test collisions.
    /// Each test suite should pick a distinct prefix so parallel test files
    /// don't create records with conflicting keys.
    static func primaryKey(for suite: String, _ label: String) -> String {
        let timestamp = Int(Date().timeIntervalSince1970)
        return "Swift \(suite) \(label) \(timestamp)"
    }
}
