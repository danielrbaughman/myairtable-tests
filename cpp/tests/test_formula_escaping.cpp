// Formula-value escaping (C# TestFormulaEscaping parity, 4 cases). Filter
// predicates build formulas by interpolating user values into quoted string
// literals; a value containing a quote, backslash, or other meta-character
// must be escaped or the formula breaks (or silently mis-matches). These tests
// round-trip special-character values through storage AND through the
// generated .eq()/.contains() filter DSL against the live base.
#include <catch2/catch_test_macros.hpp>

#include <algorithm>
#include <string>
#include <vector>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {

// Values that break naive string interpolation into an Airtable formula
// literal. Newline is covered separately (storage only) — Airtable formula
// string literals can't carry a raw newline, so it's a storage concern, not a
// filter one.
const std::vector<std::string>& special_values() {
    static const std::vector<std::string> values{
        "O'Brien single quote", "say \"hi\" double quote", "back\\slash path",
        "trailing backslash\\", "mixed a\"b'c\\d",         "unicode café ☕ 日本語 🎉",
    };
    return values;
}

bool contains_id(const std::vector<PrimaryModel>& models, const std::string& id) {
    return std::any_of(models.begin(), models.end(),
                       [&](const PrimaryModel& m) { return m.id == id; });
}

void try_remove(Airtable& airtable, const std::string& record_id) {
    if (record_id.empty()) {
        return;
    }
    try {
        airtable.primary().remove(record_id);
    } catch (const AirtableException&) {
        // best-effort cleanup
    }
}

} // namespace

TEST_CASE("formula escaping: eq filter matches value with special chars",
          "[filter][formula-escaping]") {
    auto airtable = make_airtable();
    for (const auto& special : special_values()) {
        INFO("special value: " << special);
        const auto suite = primary_key("Escaping", "Eq");
        auto created = airtable.primary().create(
            PrimaryModel{.primary_key = suite, .single_line_text = special});
        const auto record_id = *created.id;
        try {
            // 1. Storage round-trips the exact value.
            REQUIRE(created.single_line_text == special);

            // 2. The .eq() filter, scoped to this record, matches via
            //    correctly-escaped interpolation.
            const auto filter = Formulas::and_({"FIND(\"" + suite + "\", {Primary Key})",
                                                PrimaryModel::F.single_line_text.eq(special)});
            auto results = airtable.primary().get_all(AirtableQuery{.formula = filter});
            REQUIRE(contains_id(results, record_id));
            for (const auto& result : results) {
                REQUIRE(result.single_line_text == special);
            }
        } catch (...) {
            try_remove(airtable, record_id);
            throw;
        }
        try_remove(airtable, record_id);
    }
}

TEST_CASE("formula escaping: contains filter matches value with special chars",
          "[filter][formula-escaping]") {
    auto airtable = make_airtable();
    for (const auto& special : special_values()) {
        INFO("special value: " << special);
        const auto suite = primary_key("Escaping", "Contains");
        // Embed the special token inside a longer value so contains (FIND>0)
        // is a real substring test.
        const auto stored = "prefix " + special + " suffix";
        auto created = airtable.primary().create(
            PrimaryModel{.primary_key = suite, .single_line_text = stored});
        const auto record_id = *created.id;
        try {
            const auto filter =
                Formulas::and_({"FIND(\"" + suite + "\", {Primary Key})",
                                PrimaryModel::F.single_line_text.contains(special)});
            auto results = airtable.primary().get_all(AirtableQuery{.formula = filter});
            REQUIRE(contains_id(results, record_id));
        } catch (...) {
            try_remove(airtable, record_id);
            throw;
        }
        try_remove(airtable, record_id);
    }
}

TEST_CASE("formula escaping: special characters in primary key round trip and filter",
          "[filter][formula-escaping]") {
    // The primary key itself carries special chars, and we match on it with .eq().
    auto airtable = make_airtable();
    const auto suite = primary_key("Escaping", "PK");
    const auto pk = suite + " O'Brien \"quote\" back\\slash";
    auto created = airtable.primary().create(PrimaryModel{.primary_key = pk});
    const auto record_id = *created.id;
    try {
        REQUIRE(created.primary_key == pk);
        auto results = airtable.primary().get_all(
            AirtableQuery{.formula = PrimaryModel::F.primary_key.eq(pk)});
        REQUIRE(contains_id(results, record_id));
        REQUIRE(results.size() == 1);
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}

TEST_CASE("formula escaping: newline and tab values round trip through storage",
          "[filter][formula-escaping]") {
    // Newlines/tabs are a storage concern (long text), not expressible in a
    // formula literal. Verify they survive create -> fetch unchanged.
    auto airtable = make_airtable();
    const auto suite = primary_key("Escaping", "Newline");
    const std::string value = "line1\nline2\twith tab\r\nwindows";
    auto created =
        airtable.primary().create(PrimaryModel{.long_text = value, .primary_key = suite});
    const auto record_id = *created.id;
    try {
        auto fetched = airtable.primary().get(record_id);
        REQUIRE(fetched.long_text == value);
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}
