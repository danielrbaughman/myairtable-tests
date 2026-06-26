import { describe, it, expect } from "vitest";
const { TextField } = require("../output/static/formula");

// myairtable-p27e: regexMatch(pattern) must escape the pattern for the formula-string context
// (backslash FIRST, then quote) — exactly like .equals()/.contains(). Regex escapes like \d become
// \\d in the formula source, which Airtable's parser unescapes back to \d before the regex engine
// sees them. Deterministic (string-level), no network.
describe("regexMatch pattern escaping (myairtable-p27e)", () => {
	const f = new TextField("Name");

	it("escapes an embedded double quote", () => {
		expect(f.regexMatch('a"b')).toBe('REGEX({Name}, "a\\"b")');
	});

	it("doubles a backslash (so \\d survives as the regex token)", () => {
		expect(f.regexMatch("\\d")).toBe('REGEX({Name}, "\\\\d")');
	});

	it("escapes a trailing backslash so the closing quote stays escaped (the breakout case)", () => {
		expect(f.regexMatch("x\\")).toBe('REGEX({Name}, "x\\\\")');
	});
});
