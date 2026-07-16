// Runtime formula parity: create a record with all input fields, fetch it back
// with API-computed formula values, and verify the transpiled evaluate_*()
// methods reproduce them (C# TestRuntimeFormulas parity, 2 cases).
#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_string.hpp>

#include <string>

#include "test_setup.hpp"

using namespace myairtable;
using Catch::Matchers::ContainsSubstring;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {

DateTime utc(const std::string& text) {
    return *try_parse_datetime(text);
}

void try_remove_formulas(Airtable& airtable, const std::string& id) {
    try {
        airtable.formulas().delete_one(id);
    } catch (const AirtableException&) {
    }
}

/// The deterministic prefix of a date-formula rendering (before ", TODAY:",
/// which is time-dependent).
std::string before_today(const std::string& text) {
    return text.substr(0, text.find(", TODAY:"));
}

} // namespace

TEST_CASE("runtime formulas: match api", "[runtime][runtime-formulas]") {
    auto airtable = make_airtable();
    auto fresh = FormulasModel{
        .first_date = utc("2024-01-01T00:00:00.000Z"),
        .first_number = 10.0,
        .first_text = "Hello",
        .primary_key = primary_key("RuntimeFormulas", "MatchApi"),
        .second_date = utc("2024-02-01T00:00:00.000Z"),
        .second_number = 20.0,
        .second_text = "World",
        .third_date = utc("2024-03-01T00:00:00.000Z"),
        .third_number = 30.0,
        .third_text = "!",
    };
    auto created = airtable.formulas().create_one(fresh);
    const auto record_id = *created.id;
    try {
        // Re-fetch to get formula values computed by Airtable.
        auto model = airtable.formulas().get_one(record_id);

        // Math formula: exact match.
        const auto runtime_math = runtime::s(model.evaluate_math_formula());
        REQUIRE(model.math_formula.has_value());
        REQUIRE(model.math_formula->value() == runtime_math);

        // Text formula: exact match.
        const auto runtime_text = runtime::s(model.evaluate_text_formula());
        REQUIRE(model.text_formula.has_value());
        REQUIRE(model.text_formula->value() == runtime_text);

        // Date formula: deterministic parts (TODAY/TONOW/FROMNOW are time-dependent).
        REQUIRE(model.date_formula.has_value());
        const auto api_date = model.date_formula->value().value_or("");
        const auto runtime_date = runtime::s(model.evaluate_date_formula());

        REQUIRE_THAT(runtime_date, ContainsSubstring("YEAR: 2024"));
        REQUIRE_THAT(runtime_date, ContainsSubstring("MONTH: 1"));
        REQUIRE_THAT(runtime_date, ContainsSubstring("DAY: 1"));
        REQUIRE_THAT(runtime_date, ContainsSubstring("HOUR: 0"));
        REQUIRE_THAT(runtime_date, ContainsSubstring("MINUTE: 0"));
        REQUIRE_THAT(runtime_date, ContainsSubstring("SECOND: 0"));
        REQUIRE_THAT(runtime_date, ContainsSubstring("WORKDAY_DIFF: "));
        REQUIRE_THAT(runtime_date, ContainsSubstring("DATEADD+2d: 2024-01-03"));
        REQUIRE_THAT(runtime_date, ContainsSubstring("IS_BEFORE: Yes"));
        REQUIRE_THAT(runtime_date, ContainsSubstring("IS_SAME day: No"));
        REQUIRE_THAT(runtime_date, ContainsSubstring("IS_AFTER: Yes"));
        REQUIRE_THAT(runtime_date, ContainsSubstring("DATESTR: "));
        REQUIRE_THAT(runtime_date, ContainsSubstring("TIMESTR: "));

        // Compare the deterministic prefix of API vs runtime (before TODAY: which varies).
        REQUIRE(before_today(api_date) == before_today(runtime_date));
    } catch (...) {
        try_remove_formulas(airtable, record_id);
        throw;
    }
    try_remove_formulas(airtable, record_id);
}

TEST_CASE("runtime formulas: evaluate runs offline", "[runtime][runtime-formulas]") {
    // Offline smoke check: a model decoded from a synthetic fields object (with
    // dates) still evaluates every generated method without throwing.
    json fields = {
        {std::string(FormulasFields::kPrimaryKeyId), "RuntimeTest"},
        {std::string(FormulasFields::kFirstNumberId), 10},
        {std::string(FormulasFields::kSecondNumberId), 20},
        {std::string(FormulasFields::kThirdNumberId), 30},
        {std::string(FormulasFields::kFirstTextId), "Hello"},
        {std::string(FormulasFields::kSecondTextId), "World"},
        {std::string(FormulasFields::kThirdTextId), "!"},
        {std::string(FormulasFields::kFirstDateId), "2024-01-01"},
        {std::string(FormulasFields::kSecondDateId), "2024-02-01"},
        {std::string(FormulasFields::kThirdDateId), "2024-03-01"},
    };
    const json record = {{"fields", fields}};
    const auto model = record.get<FormulasModel>();

    REQUIRE_FALSE(model.evaluate_math_formula().is_null());
    const auto text = runtime::s(model.evaluate_text_formula());
    REQUIRE_THAT(text, ContainsSubstring("Hello"));
    REQUIRE_THAT(text, ContainsSubstring("World"));
    const auto date = runtime::s(model.evaluate_date_formula());
    REQUIRE_THAT(date, ContainsSubstring("YEAR: 2024"));
}
