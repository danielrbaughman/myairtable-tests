// Primary "Formula (Complex)"/"Formula (Nested)" evaluated at runtime (C#
// TestPrimaryFormulaRuntime parity, 2 cases). The base runtime suite only
// covers the Formulas table; the Primary complex formula concatenates ~35
// fields through IF(field, field, "None"), a richer transpile path.
//
// We compare the transpiled evaluate_formula_complex() to the API
// line-by-line for the DETERMINISTIC, offline-reproducible field types (text,
// checkbox, single/multi select, numbers, currency, email, url, phone). The
// formula also references server-computed fields (Created/Last Modified Time +
// By, Auto Number, Button, Formula(ID)/(Simple)) and link/lookup/rollup —
// Airtable renders those from data the offline runtime doesn't hold, so those
// lines are NOT expected to match offline. See myairtable-5b0n.
//
// This suite specifically locks in the multi-select array-join fix
// (myairtable-bb7f): a multi-value field coerces to "Option 1, Option 2", not
// just the first element.
#include <catch2/catch_test_macros.hpp>

#include <array>
#include <sstream>
#include <string>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {

// Field labels whose rendering the offline runtime can reproduce exactly.
constexpr std::array<const char*, 12> kDeterministicLabels{
    "Single Line Text",
    "Long Text",
    "Checkbox",
    "Multiple Select",
    "Single Select",
    "Number (int)",
    "Number (float)",
    "Currency (int)",
    "Currency (float)",
    "Email",
    "URL",
    "Phone Number",
};

PrimaryModel new_record(const std::string& suite) {
    return PrimaryModel{
        .checkbox = true,
        .currency_float = 9.99,
        .currency_int = 10.0,
        .email = "a@b.co",
        .long_text = "long text",
        .multiple_select =
            std::vector<PrimaryMultipleSelectOption>{PrimaryMultipleSelectOption::Option1,
                                                     PrimaryMultipleSelectOption::Option2},
        .number_float = 3.5,
        .number_int = 42.0,
        .phone_number = "555-1212",
        .primary_key = suite,
        .single_line_text = "hello",
        .single_select = PrimarySingleSelectOption::Choice1,
        .url = "https://x.co",
    };
}

/// Extract the "Label: value" line for `label` from a formula result.
std::string line_of(const std::string& formula, const std::string& label) {
    std::istringstream in(formula);
    std::string line;
    const std::string prefix = label + ": ";
    while (std::getline(in, line)) {
        if (line.rfind(prefix, 0) == 0) {
            return line;
        }
    }
    return "<missing: " + label + ">";
}

void try_remove(Airtable& airtable, const std::string& id) {
    try {
        airtable.primary().remove(id);
    } catch (const AirtableException&) {
    }
}

} // namespace

TEST_CASE("primary formula runtime: complex formula renders deterministic fields like api",
          "[runtime][primary-formula-runtime]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("PrimaryFormula", "Complex");
    auto created = airtable.primary().create(new_record(suite));
    const auto record_id = *created.id;
    try {
        auto fetched = airtable.primary().get(record_id);
        std::string api;
        if (fetched.formula_complex.has_value()) {
            const auto clean = fetched.formula_complex->clean_values();
            if (!clean.empty()) {
                api = clean.front();
            }
        }
        const auto runtime = runtime::s(fetched.evaluate_formula_complex());
        INFO("--- API ---\n" << api << "\n--- RUNTIME ---\n" << runtime);

        for (const auto* label : kDeterministicLabels) {
            REQUIRE(line_of(api, label) == line_of(runtime, label));
        }

        // The multi-select join is the headline fix: both sides render all
        // options, comma-joined.
        REQUIRE(line_of(runtime, "Multiple Select") == "Multiple Select: Option 1, Option 2");
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}

TEST_CASE("primary formula runtime: nested formula evaluates without throwing",
          "[runtime][primary-formula-runtime]") {
    // Formula (Nested) = Formula(ID) & Formula(Simple) & Formula(Complex) — it
    // chains three COMPUTED formula fields. Offline the runtime can't reproduce
    // computed-field values (they decode as special/wrapped types it doesn't
    // re-evaluate), so the content isn't asserted; this confirms the transpiled
    // nested-formula method is generated and evaluates without error. See
    // myairtable-5b0n.
    auto airtable = make_airtable();
    const auto suite = primary_key("PrimaryFormula", "Nested");
    auto created = airtable.primary().create(new_record(suite));
    const auto record_id = *created.id;
    try {
        auto fetched = airtable.primary().get(record_id);
        std::string runtime_text;
        REQUIRE_NOTHROW(runtime_text = runtime::s(fetched.evaluate_formula_nested()));
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}
