// Dict-style (untyped) CRUD against the live base via airtable.primary().dict()
// (C# TestStructCrudViaTable parity, 3 cases).
#include <catch2/catch_test_macros.hpp>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {
void try_remove(Airtable& airtable, const std::string& record_id) {
    try {
        airtable.primary().dict().remove(record_id);
    } catch (const AirtableException&) {
        // best-effort cleanup
    }
}
} // namespace

TEST_CASE("struct crud: primary-key-only round trip", "[crud][struct-crud]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("StructCrud", "PKOnly");
    Fields fields({}, PrimaryFields::kNameToId);
    fields.set_string(PrimaryFields::kPrimaryKeyId, pk);

    const auto created = airtable.primary().dict().create(fields);
    const auto record_id = created.id;
    try {
        REQUIRE_FALSE(created.id.empty());
        REQUIRE(created.fields.get_string(PrimaryFields::kPrimaryKeyId) == pk);

        const auto fetched = airtable.primary().dict().get(record_id);
        REQUIRE(fetched.id == record_id);
        REQUIRE(fetched.fields.get_string(PrimaryFields::kPrimaryKeyId) == pk);

        Fields update({}, PrimaryFields::kNameToId);
        const auto updated_pk = pk + " Updated";
        update.set_string(PrimaryFields::kPrimaryKeyId, updated_pk);
        const auto updated = airtable.primary().dict().update(record_id, update);
        REQUIRE(updated.fields.get_string(PrimaryFields::kPrimaryKeyId) == updated_pk);

        airtable.primary().dict().remove(record_id);
        REQUIRE_THROWS_AS(airtable.primary().dict().get(record_id), AirtableException);
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
}

TEST_CASE("struct crud: dual id/name field access", "[crud][struct-crud]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("StructCrud", "DualAccess");
    Fields fields({}, PrimaryFields::kNameToId);
    fields.set_string(PrimaryFields::kPrimaryKeyId, pk);

    const auto created = airtable.primary().dict().create(fields);
    try {
        REQUIRE(created.fields.get_string(PrimaryFields::kPrimaryKeyName) == pk);
        REQUIRE(created.fields.get_string(PrimaryFields::kPrimaryKeyId) == pk);
    } catch (...) {
        try_remove(airtable, created.id);
        throw;
    }
    try_remove(airtable, created.id);
}

TEST_CASE("struct crud: all simple properties round trip", "[crud][struct-crud]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("StructCrud", "AllProps");
    Fields fields({}, PrimaryFields::kNameToId);
    fields.set_string(PrimaryFields::kPrimaryKeyId, pk);
    fields.set_string(PrimaryFields::kSingleLineTextId, "Hello World");
    fields.set_string(PrimaryFields::kLongTextId, "Long text content");

    const auto created = airtable.primary().dict().create(fields);
    try {
        const auto fetched = airtable.primary().dict().get(created.id);
        REQUIRE(fetched.fields.get_string(PrimaryFields::kSingleLineTextId) == "Hello World");
        REQUIRE(fetched.fields.get_string(PrimaryFields::kLongTextId) == "Long text content");
    } catch (...) {
        try_remove(airtable, created.id);
        throw;
    }
    try_remove(airtable, created.id);
}
