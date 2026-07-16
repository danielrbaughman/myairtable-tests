// Upsert depth (C# TestUpsertDepth parity, 2 cases). The base CRUD suite only
// covers single-record, single-merge-field upsert. This adds a multi-field
// merge key (match must agree on ALL merge fields) and the multiple-match
// error path (a merge key matching more than one record is rejected by
// Airtable). Note: the generated client exposes only single-record upsert, so
// batch upsert isn't covered here.
#include <catch2/catch_test_macros.hpp>

#include <string>
#include <vector>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {

void try_remove(Airtable& airtable, const std::vector<std::string>& ids) {
    if (ids.empty()) {
        return;
    }
    try {
        airtable.primary().delete_many(ids);
    } catch (const AirtableException&) {
    }
}

} // namespace

TEST_CASE("upsert depth: upsert matches on multiple merge fields", "[crud][upsert-depth]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Upsert", "MultiKey");
    std::vector<std::string> ids;
    try {
        // Seed a record identified by the (PrimaryKey, SingleLineText) pair.
        auto seed = airtable.primary().create_one(
            PrimaryModel{.primary_key = suite, .single_line_text = "anchor"});
        ids.push_back(*seed.id);

        const std::vector<std::string> merge_on{std::string(PrimaryFields::kPrimaryKeyId),
                                                std::string(PrimaryFields::kSingleLineTextId)};

        // Same pair -> UPDATE the seed (matched on both fields).
        auto update = airtable.primary().upsert(PrimaryModel{.long_text = "updated",
                                                             .primary_key = suite,
                                                             .single_line_text = "anchor"},
                                                merge_on);
        REQUIRE_FALSE(update.was_created);
        REQUIRE(update.model.id == seed.id);
        REQUIRE(update.model.long_text == "updated");

        // Same PrimaryKey but a DIFFERENT SingleLineText -> no match on the pair -> INSERT.
        auto insert = airtable.primary().upsert(
            PrimaryModel{.primary_key = suite, .single_line_text = "different"}, merge_on);
        ids.push_back(*insert.model.id);
        REQUIRE(insert.was_created);
        REQUIRE(insert.model.id != seed.id);
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("upsert depth: upsert with multiple matches throws", "[crud][upsert-depth]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Upsert", "MultiMatch");
    std::vector<std::string> ids;
    try {
        // Two records share the same SingleLineText value.
        auto a = airtable.primary().create_one(
            PrimaryModel{.primary_key = suite + " A", .single_line_text = "dupe"});
        auto b = airtable.primary().create_one(
            PrimaryModel{.primary_key = suite + " B", .single_line_text = "dupe"});
        ids.push_back(*a.id);
        ids.push_back(*b.id);

        // Upsert merging only on SingleLineText="dupe" matches BOTH -> Airtable rejects it.
        REQUIRE_THROWS_AS(
            airtable.primary().upsert(PrimaryModel{.long_text = "x", .single_line_text = "dupe"},
                                      {std::string(PrimaryFields::kSingleLineTextId)}),
            ApiError);
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}
