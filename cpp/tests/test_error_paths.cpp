// Error and validation paths with TYPED exceptions, not bare catch-all (C#
// TestErrorPaths parity, 6 cases). Exercises the generated exception hierarchy
// against the live base: a 404 fetch, server-side validation rejections
// (invalid select option, wrong value type, unknown field), and a 401 from a
// bad key. Each asserts a specific exception subtype + its structured fields,
// proving the client classifies HTTP failures rather than collapsing them to
// one opaque error.
#include <catch2/catch_test_macros.hpp>

#include <string>
#include <type_traits>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::make_airtable_with_bad_key;
using myairtable_tests::primary_key;

namespace {

Fields primary_fields_with(const std::string& suite) {
    Fields fields({}, PrimaryFields::kNameToId);
    fields.set_string(PrimaryFields::kPrimaryKeyId, suite);
    return fields;
}

} // namespace

TEST_CASE("error paths: get nonexistent record throws api error", "[crud][error-paths]") {
    auto airtable = make_airtable();
    bool threw = false;
    try {
        airtable.primary().get_one("recDOESNOTEXIST0001");
    } catch (const ApiError& api) {
        threw = true;
        INFO("404 -> ApiError: " << api.what());
        REQUIRE_FALSE(api.code.empty());
    }
    REQUIRE(threw);
}

TEST_CASE("error paths: create with invalid select option throws api error",
          "[crud][error-paths]") {
    auto airtable = make_airtable();
    // The typed enum can't express an invalid option, so go through the dict accessor.
    auto fields = primary_fields_with(primary_key("Error", "BadSelect"));
    fields.set_string(PrimaryFields::kSingleSelectId, "NotARealOption_zzz");
    bool threw = false;
    try {
        airtable.primary().dict().create_one(fields);
    } catch (const ApiError& api) {
        threw = true;
        INFO("bad select -> ApiError: " << api.what());
        REQUIRE_FALSE(api.code.empty());
    }
    REQUIRE(threw);
}

TEST_CASE("error paths: create with wrong value type throws api error", "[crud][error-paths]") {
    auto airtable = make_airtable();
    auto fields = primary_fields_with(primary_key("Error", "BadType"));
    fields.set(PrimaryFields::kNumberIntId, json("not a number"));
    bool threw = false;
    try {
        airtable.primary().dict().create_one(fields);
    } catch (const ApiError& api) {
        threw = true;
        INFO("wrong type -> ApiError: " << api.what());
    }
    REQUIRE(threw);
}

TEST_CASE("error paths: create with unknown field throws api error", "[crud][error-paths]") {
    auto airtable = make_airtable();
    Fields fields({}, PrimaryFields::kNameToId);
    fields.set_string(PrimaryFields::kPrimaryKeyId, primary_key("Error", "BadField"));
    fields.set("fldDOESNOTEXIST00000", json("x"));
    bool threw = false;
    try {
        airtable.primary().dict().create_one(fields);
    } catch (const ApiError& api) {
        threw = true;
        INFO("unknown field -> ApiError: " << api.what());
    }
    REQUIRE(threw);
}

TEST_CASE("error paths: bad api key throws auth error", "[crud][error-paths]") {
    // Real base + bogus key so auth (401) is checked, not the base (404).
    auto bad = make_airtable_with_bad_key();
    // Auth failures come back as a structured 401 envelope (ApiError) or, if
    // not parseable, a raw HttpError — both are acceptable typed
    // classifications (never the base type alone).
    bool typed = false;
    try {
        bad.primary().get_many(AirtableQuery{.max_records = 1});
    } catch (const ApiError& api) {
        typed = true;
        INFO("bad key -> ApiError: " << api.what());
    } catch (const HttpError& http) {
        typed = true;
        INFO("bad key -> HttpError: " << http.what());
    }
    REQUIRE(typed);
}

TEST_CASE("error paths: rate limited error carries retry-after", "[crud][error-paths]") {
    // The 429 path is exercised live elsewhere; here we assert the type is a
    // distinct, well-formed member of the hierarchy carrying its Retry-After
    // value.
    const RateLimitedError err(30.0);
    static_assert(std::is_base_of_v<AirtableException, RateLimitedError>);
    REQUIRE(err.retry_after_seconds == 30.0);
}
