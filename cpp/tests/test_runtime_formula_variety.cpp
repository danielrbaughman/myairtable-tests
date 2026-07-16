// Runtime-formula input variety (C# TestRuntimeFormulaVariety parity). The base
// runtime-formulas suite only evaluates the kitchen-sink formulas with ONE
// fully-populated input set, so the IF(OR(...=BLANK())) short-circuit and
// varied inputs are never exercised. Each case creates a Formulas record with a
// specific input set, fetches the API-computed value, and asserts the
// transpiled evaluate_*() reproduces it.
//
// Scope notes (kept deliberately portable across all targets):
//  - Numbers are positive with LOG/SQRT arguments chosen to be exact (powers of
//    10 / perfect squares). The Math formula calls LOG()/SQRT()/EXP() —
//    transcendental results differ by a ULP between platforms' math libs and
//    Airtable (V8), so irrational results (e.g. LOG(5)) are NOT bit-identical
//    and would make an exact string compare flaky. Negatives/zero are excluded
//    too: LOG(negative) and MOD(_,0) error inside this formula. See
//    myairtable-5b0n.
//  - Text covers empty-ish edges (whitespace), unicode, and reserved
//    punctuation (exercising the fixed ENCODE_URL_COMPONENT). An all-blank text
//    input is excluded: Airtable returns blank for the whole formula (REPLACE
//    past end-of-string errors), while the transpiler is lenient.
#include <catch2/catch_test_macros.hpp>
#include <catch2/generators/catch_generators.hpp>

#include <string>
#include <tuple>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {

DateTime utc(const std::string& text) {
    return *try_parse_datetime(text);
}

FormulasModel base_model(const std::string& label) {
    return FormulasModel{
        .first_date = utc("2024-01-01T00:00:00.000Z"),
        .primary_key = primary_key("Variety", label),
        .second_date = utc("2024-02-01T00:00:00.000Z"),
        .third_date = utc("2024-03-01T00:00:00.000Z"),
    };
}

void try_remove_formulas(Airtable& airtable, const std::string& id) {
    try {
        airtable.formulas().delete_one(id);
    } catch (const AirtableException&) {
    }
}

} // namespace

TEST_CASE("runtime formula variety: math formula matches api for varied numbers",
          "[runtime][runtime-formula-variety]") {
    // First Number: power of 10 (LOG exact). Second Number: perfect square (SQRT exact).
    const auto [label, a, b, c] = GENERATE(table<std::string, double, double, double>({
        {"hundreds", 100.0, 16.0, 8.0},
        {"ones", 1.0, 4.0, 2.0},
        {"tens", 10.0, 25.0, 3.0},
    }));
    auto airtable = make_airtable();
    auto model = base_model("Math " + label);
    model.first_number = a;
    model.second_number = b;
    model.third_number = c;
    model.first_text = "x";
    model.second_text = "y";
    model.third_text = "z";
    auto created = airtable.formulas().create_one(model);
    const auto record_id = *created.id;
    try {
        auto fetched = airtable.formulas().get_one(record_id);
        const auto runtime = runtime::s(fetched.evaluate_math_formula());
        INFO(label << ": api='"
                   << (fetched.math_formula ? fetched.math_formula->value().value_or("") : "")
                   << "' runtime='" << runtime << "'");
        REQUIRE(fetched.math_formula.has_value());
        REQUIRE(fetched.math_formula->value() == runtime);
    } catch (...) {
        try_remove_formulas(airtable, record_id);
        throw;
    }
    try_remove_formulas(airtable, record_id);
}

TEST_CASE("runtime formula variety: math formula blank branch when numbers missing",
          "[runtime][runtime-formula-variety]") {
    // First/Second Number left null -> OR(BLANK, BLANK) is true -> the formula
    // returns BLANK(). This is the IF-true short-circuit that the base suite
    // never reaches.
    auto airtable = make_airtable();
    auto model = base_model("Blank");
    model.first_text = "x";
    model.second_text = "y";
    model.third_text = "z";
    auto created = airtable.formulas().create_one(model);
    const auto record_id = *created.id;
    try {
        auto fetched = airtable.formulas().get_one(record_id);
        const auto api_val = fetched.math_formula ? fetched.math_formula->value().value_or("") : "";
        const auto runtime = runtime::s(fetched.evaluate_math_formula());
        INFO("blank: api='" << api_val << "' runtime='" << runtime << "'");
        REQUIRE(api_val.empty());
        REQUIRE(runtime.empty());
    } catch (...) {
        try_remove_formulas(airtable, record_id);
        throw;
    }
    try_remove_formulas(airtable, record_id);
}

TEST_CASE("runtime formula variety: text formula matches api for varied text",
          "[runtime][runtime-formula-variety]") {
    const auto [label, a, b, c] =
        GENERATE(table<std::string, std::string, std::string, std::string>({
            {"unicode", "café", "naïve", "日本語🎉"},
            {"whitespace", "  he llo  ", "a b", "c"},
            {"punct", "a.e-i+o", "x/y", "z"}, // exercises fixed ENCODE_URL_COMPONENT
        }));
    auto airtable = make_airtable();
    auto model = base_model("Text " + label);
    model.first_number = 10.0;
    model.second_number = 20.0;
    model.third_number = 30.0;
    model.first_text = a;
    model.second_text = b;
    model.third_text = c;
    auto created = airtable.formulas().create_one(model);
    const auto record_id = *created.id;
    try {
        auto fetched = airtable.formulas().get_one(record_id);
        const auto runtime = runtime::s(fetched.evaluate_text_formula());
        INFO(label << ": api='"
                   << (fetched.text_formula ? fetched.text_formula->value().value_or("") : "")
                   << "' runtime='" << runtime << "'");
        REQUIRE(fetched.text_formula.has_value());
        REQUIRE(fetched.text_formula->value() == runtime);
    } catch (...) {
        try_remove_formulas(airtable, record_id);
        throw;
    }
    try_remove_formulas(airtable, record_id);
}
