/**
 * Deterministic (offline) tests for the typed error hierarchy (myairtable-9yrt) — JavaScript twin.
 *
 * Exercises the two classifiers + the typed classes exported from the generated output:
 *   - classifyAirtableJsError: maps an airtable.js-shaped `{ error, message, statusCode }` to a
 *     typed AirtableError (idempotent for an already-typed error).
 *   - classifyHttpResponse: maps a raw-fetch status + body (+ optional Retry-After) to a typed error,
 *     parsing BOTH envelope shapes (structured `{"error":{"type":...}}` and legacy `{"error":"CODE"}`).
 * No network is touched.
 */
import { describe, it, expect } from "vitest";
const {
	AirtableError,
	ApiError,
	RateLimitedError,
	HttpError,
	NetworkError,
	classifyAirtableJsError,
	classifyHttpResponse,
} = require("../output");

describe("classifyAirtableJsError", () => {
	it("maps a structured airtable.js error to ApiError", () => {
		const err = classifyAirtableJsError({ error: "NOT_FOUND", statusCode: 404, message: "x" });
		expect(err).toBeInstanceOf(ApiError);
		expect(err).toBeInstanceOf(AirtableError);
		expect(err).toBeInstanceOf(Error);
		expect(err.code).toBe("NOT_FOUND");
		expect(err.statusCode).toBe(404);
	});

	it("maps a 429 to RateLimitedError (no Retry-After from airtable.js)", () => {
		const err = classifyAirtableJsError({ statusCode: 429 });
		expect(err).toBeInstanceOf(RateLimitedError);
		expect(err).toBeInstanceOf(AirtableError);
		expect(err.retryAfter).toBeUndefined();
	});

	it("maps a status without an error code to HttpError", () => {
		const err = classifyAirtableJsError({ statusCode: 500, message: "boom" });
		expect(err).toBeInstanceOf(HttpError);
		expect(err.statusCode).toBe(500);
	});

	it("maps a status-less (transport) failure to NetworkError", () => {
		const err = classifyAirtableJsError("ECONNRESET");
		expect(err).toBeInstanceOf(NetworkError);
		expect(err).toBeInstanceOf(AirtableError);
	});

	it("is idempotent: an already-typed error is returned unchanged", () => {
		const original = new ApiError("X", 400, "m");
		expect(classifyAirtableJsError(original)).toBe(original);
	});
});

describe("classifyHttpResponse", () => {
	it("parses the structured envelope into an ApiError", () => {
		const err = classifyHttpResponse(422, '{"error":{"type":"INVALID_VALUE_FOR_COLUMN","message":"m"}}');
		expect(err).toBeInstanceOf(ApiError);
		expect(err.code).toBe("INVALID_VALUE_FOR_COLUMN");
		expect(err.statusCode).toBe(422);
	});

	it("parses the legacy bare-string envelope into an ApiError", () => {
		const err = classifyHttpResponse(404, '{"error":"NOT_FOUND"}');
		expect(err).toBeInstanceOf(ApiError);
		expect(err.code).toBe("NOT_FOUND");
	});

	it("maps a 429 to RateLimitedError carrying the parsed Retry-After", () => {
		const err = classifyHttpResponse(429, "{}", 12);
		expect(err).toBeInstanceOf(RateLimitedError);
		expect(err.retryAfter).toBe(12);
	});

	it("falls back to HttpError for an unparseable body", () => {
		const err = classifyHttpResponse(503, "upstream down");
		expect(err).toBeInstanceOf(HttpError);
		expect(err.statusCode).toBe(503);
	});
});
