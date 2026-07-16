// Multi-page pagination (C# TestPagination parity, 4 cases): list more records
// than Airtable's 100-record default page in a single query (no maxRecords)
// and assert the client's offset loop reassembles every page, plus explicit
// pageSize and pageSize+maxRecords interplay.
#include <catch2/catch_test_macros.hpp>

#include <set>
#include <string>
#include <vector>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {

// 105 > the 100-record default page size, so a no-maxRecords list spans two pages.
constexpr int kCount = 105;

// Explicit page size: 25 records over pageSize 10 spans 3 pages (10+10+5).
constexpr int kPageSizeCount = 25;

std::string find_formula(const std::string& suite) {
    return "FIND(\"" + suite + "\", {Primary Key})";
}

/// Best-effort cleanup: batch delete by id, falling back to a FIND prefix sweep.
void try_delete_many(Airtable& airtable, const std::vector<std::string>& ids,
                     const std::string& suite) {
    if (!ids.empty()) {
        try {
            airtable.primary().dict().remove(ids);
            return;
        } catch (const AirtableException&) {
            // fall through to a prefix sweep
        }
    }
    try {
        auto stray =
            airtable.primary().dict().get_all(AirtableQuery{.formula = find_formula(suite)});
        if (!stray.empty()) {
            std::vector<std::string> stray_ids;
            stray_ids.reserve(stray.size());
            for (const auto& record : stray) {
                stray_ids.push_back(record.id);
            }
            airtable.primary().dict().remove(stray_ids);
        }
    } catch (const AirtableException&) {
        // best-effort cleanup
    }
}

} // namespace

TEST_CASE("pagination: orm get spanning multiple pages returns every record",
          "[crud][pagination]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Pagination", "Orm");
    std::vector<PrimaryModel> models;
    for (int i = 1; i <= kCount; ++i) {
        models.push_back(PrimaryModel{.primary_key = suite + " " + std::to_string(i)});
    }
    std::vector<std::string> created_ids;
    try {
        auto created = airtable.primary().create(models);
        for (const auto& m : created) {
            created_ids.push_back(*m.id);
        }
        REQUIRE(created.size() == kCount);

        // No maxRecords: the offset loop must walk both pages and return all 105.
        auto results = airtable.primary().get_all(AirtableQuery{.formula = find_formula(suite)});
        REQUIRE(results.size() == kCount);
        std::set<std::string> expected(created_ids.begin(), created_ids.end());
        std::set<std::string> actual;
        for (const auto& m : results) {
            actual.insert(*m.id);
        }
        REQUIRE(actual == expected);
    } catch (...) {
        try_delete_many(airtable, created_ids, suite);
        throw;
    }
    try_delete_many(airtable, created_ids, suite);
}

TEST_CASE("pagination: dict get spanning multiple pages returns every record",
          "[crud][pagination]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Pagination", "Dict");
    std::vector<Fields> creates;
    for (int i = 1; i <= kCount; ++i) {
        Fields fields({}, PrimaryFields::kNameToId);
        fields.set_string(PrimaryFields::kPrimaryKeyId, suite + " " + std::to_string(i));
        creates.push_back(std::move(fields));
    }
    std::vector<std::string> created_ids;
    try {
        auto created = airtable.primary().dict().create(creates);
        for (const auto& record : created) {
            created_ids.push_back(record.id);
        }
        REQUIRE(created.size() == kCount);

        auto results =
            airtable.primary().dict().get_all(AirtableQuery{.formula = find_formula(suite)});
        REQUIRE(results.size() == kCount);
        std::set<std::string> expected(created_ids.begin(), created_ids.end());
        std::set<std::string> actual;
        for (const auto& record : results) {
            actual.insert(record.id);
        }
        REQUIRE(actual == expected);
    } catch (...) {
        try_delete_many(airtable, created_ids, suite);
        throw;
    }
    try_delete_many(airtable, created_ids, suite);
}

TEST_CASE("pagination: explicit page size returns all records across pages", "[crud][pagination]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Pagination", "PageSize");
    std::vector<PrimaryModel> models;
    for (int i = 1; i <= kPageSizeCount; ++i) {
        models.push_back(PrimaryModel{.primary_key = suite + " " + std::to_string(i)});
    }
    std::vector<std::string> created_ids;
    try {
        auto created = airtable.primary().create(models);
        for (const auto& m : created) {
            created_ids.push_back(*m.id);
        }

        // pageSize 10, NO maxRecords: the offset loop must walk all 3 pages
        // and return all 25.
        auto results = airtable.primary().get_all(
            AirtableQuery{.formula = find_formula(suite), .page_size = 10});
        REQUIRE(results.size() == kPageSizeCount);
    } catch (...) {
        try_delete_many(airtable, created_ids, suite);
        throw;
    }
    try_delete_many(airtable, created_ids, suite);
}

TEST_CASE("pagination: page size with max records caps the total", "[crud][pagination]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Pagination", "PageCap");
    std::vector<PrimaryModel> models;
    for (int i = 1; i <= kPageSizeCount; ++i) {
        models.push_back(PrimaryModel{.primary_key = suite + " " + std::to_string(i)});
    }
    std::vector<std::string> created_ids;
    try {
        auto created = airtable.primary().create(models);
        for (const auto& m : created) {
            created_ids.push_back(*m.id);
        }

        // pageSize 10 + maxRecords 15: maxRecords caps the total mid-stream
        // (not a page multiple).
        auto results = airtable.primary().get_all(
            AirtableQuery{.formula = find_formula(suite), .max_records = 15, .page_size = 10});
        REQUIRE(results.size() == 15);
    } catch (...) {
        try_delete_many(airtable, created_ids, suite);
        throw;
    }
    try_delete_many(airtable, created_ids, suite);
}
