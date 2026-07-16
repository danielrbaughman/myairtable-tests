// Multi-field sort + sort combined with a filter (C# TestMultiFieldSort
// parity, 3 cases). The base filter suite only covers a single-field sort;
// this verifies a two-field sort (ties broken by a secondary key) and sorting
// within a filtered scope.
#include <catch2/catch_test_macros.hpp>

#include <string>
#include <vector>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {

Fields row(const std::string& suite, int64_t number, const std::string& text) {
    Fields fields({}, PrimaryFields::kNameToId);
    fields.set_string(PrimaryFields::kPrimaryKeyId, suite + " " + text);
    fields.set_long(PrimaryFields::kNumberIntId, number);
    fields.set_string(PrimaryFields::kSingleLineTextId, text);
    return fields;
}

std::string scope_to(const std::vector<std::string>& ids) {
    std::vector<std::string> parts;
    parts.reserve(ids.size());
    for (const auto& id : ids) {
        parts.push_back("RECORD_ID()='" + id + "'");
    }
    return Formulas::or_(parts);
}

std::vector<std::string> texts_of(const std::vector<DictTable::Record>& records) {
    std::vector<std::string> out;
    out.reserve(records.size());
    for (const auto& record : records) {
        out.push_back(*record.fields.get_string(PrimaryFields::kSingleLineTextId));
    }
    return out;
}

std::vector<std::string> ids_of(const std::vector<DictTable::Record>& records) {
    std::vector<std::string> ids;
    ids.reserve(records.size());
    for (const auto& record : records) {
        ids.push_back(record.id);
    }
    return ids;
}

void try_remove(Airtable& airtable, const std::vector<std::string>& ids) {
    if (ids.empty()) {
        return;
    }
    try {
        airtable.primary().dict().delete_many(ids);
    } catch (const AirtableException&) {
    }
}

} // namespace

TEST_CASE("multi field sort: two-field sort breaks ties on second key",
          "[filter][multi-field-sort]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Sort", "TwoField");
    // NumberInt ties at 10 (rows "b" and "a"); the secondary SingleLineText sort orders them.
    auto created = airtable.primary().dict().create_many(
        std::vector<Fields>{row(suite, 10, "b"), row(suite, 10, "a"), row(suite, 20, "c")});
    const auto ids = ids_of(created);
    try {
        auto results = airtable.primary().dict().get_many(
            AirtableQuery{.formula = scope_to(ids),
                          .sorts = {{.field = std::string(PrimaryFields::kNumberIntId),
                                     .direction = SortDirection::Asc},
                                    {.field = std::string(PrimaryFields::kSingleLineTextId),
                                     .direction = SortDirection::Asc}}});
        // (10,a), (10,b), (20,c) — tie on 10 broken by text asc.
        REQUIRE(texts_of(results) == std::vector<std::string>{"a", "b", "c"});
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("multi field sort: secondary descending reverses tied group",
          "[filter][multi-field-sort]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Sort", "MixedDir");
    auto created = airtable.primary().dict().create_many(
        std::vector<Fields>{row(suite, 10, "a"), row(suite, 10, "b"), row(suite, 20, "c")});
    const auto ids = ids_of(created);
    try {
        auto results = airtable.primary().dict().get_many(
            AirtableQuery{.formula = scope_to(ids),
                          .sorts = {{.field = std::string(PrimaryFields::kNumberIntId),
                                     .direction = SortDirection::Asc},
                                    {.field = std::string(PrimaryFields::kSingleLineTextId),
                                     .direction = SortDirection::Desc}}});
        // NumberInt asc, then text DESC within the 10-tie: (10,b), (10,a), (20,c).
        REQUIRE(texts_of(results) == std::vector<std::string>{"b", "a", "c"});
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("multi field sort: sort combined with a filter", "[filter][multi-field-sort]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Sort", "WithFilter");
    auto created = airtable.primary().dict().create_many(std::vector<Fields>{
        row(suite, 30, "x"),
        row(suite, 10, "y"),
        row(suite, 20, "z"),
        row(suite, 5, "low"), // filtered out by NumberInt > 5
    });
    const auto ids = ids_of(created);
    try {
        const auto filter =
            Formulas::and_({scope_to(ids), PrimaryModel::F.number_int.greater_than(5)});
        auto results = airtable.primary().dict().get_many(
            AirtableQuery{.formula = filter,
                          .sorts = {{.field = std::string(PrimaryFields::kNumberIntId),
                                     .direction = SortDirection::Asc}}});
        // Filtered to NumberInt > 5, sorted asc: 10(y), 20(z), 30(x).
        REQUIRE(texts_of(results) == std::vector<std::string>{"y", "z", "x"});
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}
