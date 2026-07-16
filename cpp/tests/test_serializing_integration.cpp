// Network-bound serialization cases, split from the offline test_serializing
// suite (C# TestSerializingIntegration parity, 3 cases): id + createdTime
// round-trip after fetch, linked-record fields serializing as record-ID
// arrays, and an explicit null clearing a writable field server-side.
#include <catch2/catch_test_macros.hpp>

#include <string>
#include <vector>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {

void try_delete_primary(Airtable& airtable, const std::string& id) {
    if (id.empty()) {
        return;
    }
    try {
        airtable.primary().delete_one(id);
    } catch (const AirtableException&) {
    }
}

void try_delete_secondary(Airtable& airtable, const std::string& id) {
    if (id.empty()) {
        return;
    }
    try {
        airtable.secondary().delete_one(id);
    } catch (const AirtableException&) {
    }
}

} // namespace

TEST_CASE("serializing integration: round trip preserves id and created time after fetch",
          "[json][serializing-integration]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("Serializing", "IdTime");
    auto created = airtable.primary().create_one(PrimaryModel{.primary_key = pk});
    const auto record_id = *created.id;
    try {
        REQUIRE(created.created_time.has_value()); // populated on create
        auto fetched = airtable.primary().get_one(record_id);
        REQUIRE(fetched.id == record_id);
        REQUIRE(fetched.created_time == created.created_time);
    } catch (...) {
        try_delete_primary(airtable, record_id);
        throw;
    }
    try_delete_primary(airtable, record_id);
}

TEST_CASE("serializing integration: linked record fields serialize as record id arrays",
          "[json][serializing-integration]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Serializing", "Links");
    std::string sec_id;
    std::string prim_id;
    try {
        auto sec = airtable.secondary().create_one(SecondaryModel{.name = suite + " Target"});
        sec_id = *sec.id;
        auto prim = airtable.primary().create_one(
            PrimaryModel{.link_single = std::vector<std::string>{sec_id}, .primary_key = suite});
        prim_id = *prim.id;

        const auto record = prim.to_record();
        REQUIRE(record.contains(std::string(PrimaryFields::kLinkSingleId)));
        const auto& link_element = record.at(std::string(PrimaryFields::kLinkSingleId));
        REQUIRE(link_element.is_array()); // links serialize as arrays
        REQUIRE(link_element[0].get<std::string>() == sec_id);
    } catch (...) {
        try_delete_primary(airtable, prim_id);
        try_delete_secondary(airtable, sec_id);
        throw;
    }
    try_delete_primary(airtable, prim_id);
    try_delete_secondary(airtable, sec_id);
}

TEST_CASE("serializing integration: explicit null on a writable field clears it server side",
          "[json][serializing-integration]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("Serializing", "Clear");
    auto created = airtable.primary().create_one(
        PrimaryModel{.primary_key = pk, .single_line_text = "to be cleared"});
    const auto record_id = *created.id;
    try {
        created.single_line_text = std::nullopt;
        auto saved = airtable.primary().update_one(created);
        REQUIRE(saved.single_line_text == std::nullopt); // null dirty entry clears the field

        auto fetched = airtable.primary().get_one(record_id);
        REQUIRE(fetched.single_line_text == std::nullopt);
    } catch (...) {
        try_delete_primary(airtable, record_id);
        throw;
    }
    try_delete_primary(airtable, record_id);
}
