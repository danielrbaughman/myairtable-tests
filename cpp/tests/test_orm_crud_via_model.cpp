// Model-side fluent CRUD — save()/fetch()/remove() on the model itself
// (C# TestOrmCrudViaModel parity, 4 cases).
#include <catch2/catch_test_macros.hpp>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {
void try_remove(Airtable& airtable, const std::string& record_id) {
    try {
        airtable.primary().delete_one(record_id);
    } catch (const AirtableException&) {
    }
}
} // namespace

TEST_CASE("model crud: primary-key-only crud via model methods", "[crud][orm-model]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("OrmModel", "PKOnly");
    auto created = airtable.primary().create_one(PrimaryModel{.primary_key = pk});
    const auto record_id = *created.id;
    try {
        // fetch() returns a fresh instance
        auto fetched = created.fetch();
        REQUIRE(fetched.id == record_id);
        REQUIRE(fetched.primary_key == pk);

        // save() persists dirty fields
        fetched.primary_key = pk + " ViaModel";
        auto saved = fetched.save();
        REQUIRE(saved.primary_key == pk + " ViaModel");
        REQUIRE(airtable.primary().get_one(record_id).primary_key == pk + " ViaModel");

        // remove() deletes the record
        saved.remove();
        REQUIRE_THROWS_AS(airtable.primary().get_one(record_id), AirtableException);
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
}

TEST_CASE("model crud: dirty tracking resets after a successful save", "[crud][orm-model]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("OrmModel", "DirtyReset");
    auto created = airtable.primary().create_one(PrimaryModel{.primary_key = pk});
    const auto record_id = *created.id;
    try {
        auto model = airtable.primary().get_one(record_id);
        REQUIRE(model.dirty_fields().empty());

        model.single_line_text = "changed";
        REQUIRE(model.dirty_fields().size() == 1);

        auto saved = model.save();
        REQUIRE(saved.dirty_fields().empty()); // fresh snapshot on the returned instance
        REQUIRE(saved.single_line_text == "changed");
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}

TEST_CASE("model crud: remove on an unsaved model throws", "[crud][orm-model]") {
    auto fresh = PrimaryModel{.primary_key = "never persisted"};
    REQUIRE_THROWS_AS(fresh.remove(), AirtableException);
}

TEST_CASE("model crud: fetch on an unsaved model throws", "[crud][orm-model]") {
    auto fresh = PrimaryModel{.primary_key = "never persisted"};
    REQUIRE_THROWS_AS(fresh.fetch(), AirtableException);
}
