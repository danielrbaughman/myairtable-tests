// TTL cache behavior end-to-end: hits, default-off, mutation invalidation,
// manual invalidation, TTL expiry, query-key separation (C# TestCaching
// parity, 9 cases).
#include <catch2/catch_test_macros.hpp>

#include <chrono>
#include <string>
#include <thread>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {

Airtable cached() {
    return make_airtable(60.0);
}

Airtable uncached() {
    return make_airtable(); // cache_seconds defaults to 0
}

void best_effort_delete(Airtable& at, const std::string& id) {
    if (id.empty()) {
        return;
    }
    try {
        at.primary().dict().delete_one(id);
    } catch (const AirtableException&) {
    }
}

} // namespace

TEST_CASE("caching: cache hit returns same record on repeated get", "[cache][caching]") {
    auto at = cached();
    auto created =
        at.primary().create_one(PrimaryModel{.primary_key = primary_key("Cache", "Hit")});
    const auto id = *created.id;
    try {
        auto first = at.primary().get_one(id);
        auto second = at.primary().get_one(id);
        REQUIRE(first.primary_key == second.primary_key);
        REQUIRE(first.id == second.id);
        INFO("read populated the cache");
        REQUIRE(at.client()->cache().size() > 0);
    } catch (...) {
        best_effort_delete(at, id);
        throw;
    }
    best_effort_delete(at, id);
}

TEST_CASE("caching: cache is disabled when cache seconds is zero", "[cache][caching]") {
    auto at = uncached();
    REQUIRE(at.client()->cache().size() == 0);
    auto created =
        at.primary().create_one(PrimaryModel{.primary_key = primary_key("Cache", "Off")});
    const auto id = *created.id;
    try {
        at.primary().get_one(id);
        REQUIRE(at.client()->cache().size() == 0); // TTL=0 is a no-op
    } catch (...) {
        best_effort_delete(at, id);
        throw;
    }
    best_effort_delete(at, id);
}

TEST_CASE("caching: create invalidates cached reads for the table", "[cache][caching]") {
    auto at = cached();
    const auto key = primary_key("Cache", "MutateCreate");
    auto a = at.primary().create_one(PrimaryModel{.primary_key = key});
    const auto id_a = *a.id;
    std::string id_b;
    try {
        at.primary().get_one(id_a); // populate cache
        REQUIRE(at.client()->cache().size() > 0);

        auto b = at.primary().create_one(PrimaryModel{.primary_key = key + " B"});
        id_b = *b.id;
        REQUIRE(at.client()->cache().size() == 0); // create wipes this table's cache
    } catch (...) {
        best_effort_delete(at, id_a);
        best_effort_delete(at, id_b);
        throw;
    }
    best_effort_delete(at, id_a);
    best_effort_delete(at, id_b);
}

TEST_CASE("caching: update invalidates cached reads for the table", "[cache][caching]") {
    auto at = cached();
    auto created =
        at.primary().create_one(PrimaryModel{.primary_key = primary_key("Cache", "MutateUpdate")});
    const auto id = *created.id;
    try {
        at.primary().get_one(id);
        REQUIRE(at.client()->cache().size() > 0);

        created.single_line_text = "mutated";
        at.primary().update_one(created);
        REQUIRE(at.client()->cache().size() == 0); // update wipes this table's cache
    } catch (...) {
        best_effort_delete(at, id);
        throw;
    }
    best_effort_delete(at, id);
}

TEST_CASE("caching: delete invalidates cached reads for the table", "[cache][caching]") {
    auto at = cached();
    auto created =
        at.primary().create_one(PrimaryModel{.primary_key = primary_key("Cache", "MutateDelete")});
    const auto id = *created.id;
    bool deleted = false;
    try {
        at.primary().get_one(id);
        REQUIRE(at.client()->cache().size() > 0);

        at.primary().delete_one(id);
        deleted = true;
        REQUIRE(at.client()->cache().size() == 0); // delete wipes this table's cache
    } catch (...) {
        if (!deleted) {
            best_effort_delete(at, id);
        }
        throw;
    }
}

TEST_CASE("caching: invalidate cache drops this table's cache", "[cache][caching]") {
    auto at = cached();
    auto created =
        at.primary().create_one(PrimaryModel{.primary_key = primary_key("Cache", "ManualTable")});
    const auto id = *created.id;
    try {
        at.primary().get_one(id);
        REQUIRE(at.client()->cache().size() > 0);

        at.client()->invalidate_cache(std::string(PrimaryModel::kTableId));
        REQUIRE(at.client()->cache().size() == 0);
    } catch (...) {
        best_effort_delete(at, id);
        throw;
    }
    best_effort_delete(at, id);
}

TEST_CASE("caching: invalidate all caches drops every table's cache", "[cache][caching]") {
    auto at = cached();
    auto created =
        at.primary().create_one(PrimaryModel{.primary_key = primary_key("Cache", "ManualAll")});
    const auto id = *created.id;
    try {
        at.primary().get_one(id);
        at.secondary().get_many(AirtableQuery{.max_records = 1});
        REQUIRE(at.client()->cache().size() >= 2);

        at.invalidate_all_caches();
        REQUIRE(at.client()->cache().size() == 0);
    } catch (...) {
        best_effort_delete(at, id);
        throw;
    }
    best_effort_delete(at, id);
}

TEST_CASE("caching: cached entries expire after the configured ttl", "[cache][caching]") {
    auto at = make_airtable(1.0);
    const auto key = primary_key("Cache", "TTL");
    auto created = at.primary().create_one(PrimaryModel{.primary_key = key});
    const auto id = *created.id;
    try {
        at.primary().get_one(id);
        REQUIRE(at.client()->cache().size() > 0);

        std::this_thread::sleep_for(std::chrono::milliseconds(1500));
        // Probe the single-record entry directly: cache().get lazily evicts
        // past TTL and returns nullopt, proving the entry expired rather than
        // being served stale. Key mirrors AirtableClient::get_record:
        // "rec:" + id.
        REQUIRE(at.client()->cache().get(std::string(PrimaryModel::kTableId), "rec:" + id) ==
                std::nullopt);

        auto fresh = at.primary().get_one(id);
        REQUIRE(fresh.primary_key == key);
    } catch (...) {
        best_effort_delete(at, id);
        throw;
    }
    best_effort_delete(at, id);
}

TEST_CASE("caching: different query params produce different cache keys", "[cache][caching]") {
    auto at = cached();
    auto created =
        at.primary().create_one(PrimaryModel{.primary_key = primary_key("Cache", "Keys")});
    const auto id = *created.id;
    try {
        at.primary().get_many(AirtableQuery{.max_records = 1});
        at.primary().get_many(AirtableQuery{.max_records = 2});
        INFO("distinct queries cache separately");
        REQUIRE(at.client()->cache().size() >= 2);
    } catch (...) {
        best_effort_delete(at, id);
        throw;
    }
    best_effort_delete(at, id);
}
