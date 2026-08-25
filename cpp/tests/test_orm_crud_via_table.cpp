// Typed ORM CRUD via the table accessor — airtable.primary()
// (C# TestOrmCrudViaTable parity, 8 cases).
#include <catch2/catch_test_macros.hpp>

#include <set>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {
void try_remove(Airtable& airtable, const std::vector<std::string>& ids) {
    try {
        airtable.primary().delete_many(ids);
    } catch (const AirtableException&) {
    }
}

void try_sweep_by_prefix(Airtable& airtable, const std::string& prefix) {
    try {
        airtable.primary().delete_all(
            AirtableQuery{.formula = "FIND(\"" + prefix + "\", {Primary Key})"});
    } catch (const AirtableException&) {
    }
}
} // namespace

TEST_CASE("orm table: primary-key-only crud round trip", "[crud][orm-table]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("OrmTable", "PKOnly");
    auto created = airtable.primary().create_one(PrimaryModel{.primary_key = pk});
    REQUIRE(created.id.has_value());
    REQUIRE(created.primary_key == pk);

    const auto record_id = *created.id;
    try {
        auto fetched = airtable.primary().get_one(record_id);
        REQUIRE(fetched.id == record_id);
        REQUIRE(fetched.primary_key == pk);
        REQUIRE(fetched.dirty_fields().empty());

        fetched.primary_key = pk + " Updated";
        REQUIRE_FALSE(fetched.dirty_fields().empty());
        auto updated = airtable.primary().update_one(fetched);
        REQUIRE(updated.primary_key == pk + " Updated");

        airtable.primary().delete_one(record_id);
        REQUIRE_THROWS_AS(airtable.primary().get_one(record_id), AirtableException);
    } catch (...) {
        try_remove(airtable, {record_id});
        throw;
    }
}

TEST_CASE("orm table: all simple properties round trip", "[crud][orm-table]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("OrmTable", "AllProps");
    auto model = PrimaryModel{
        .primary_key = pk,
        .single_line_text = "Hello World",
        .long_text = "Long text content",
        .email = "test@example.com",
        .url = "https://example.com",
        .phone_number = "555-1234",
        .checkbox = true,
        .number_int = 42.0,
        .number_float = 3.14,
        .currency_int = 10.0,
        .currency_float = 9.99,
        .percent_int = 0.5,
        .percent_float = 0.333,
        .duration = Duration{3600.0},
    };
    auto created = airtable.primary().create_one(model);
    const auto record_id = *created.id;
    try {
        REQUIRE(created.primary_key == pk);
        REQUIRE(created.single_line_text == "Hello World");
        REQUIRE(created.email == "test@example.com");
        REQUIRE(created.checkbox == true);
        REQUIRE(created.number_int == 42.0);
        REQUIRE(created.number_float == 3.14);
        REQUIRE(created.currency_float == 9.99);
        REQUIRE(created.url == "https://example.com");
        REQUIRE(created.phone_number == "555-1234");
        REQUIRE(created.duration == Duration{3600.0}); // duration wire unit is seconds
    } catch (...) {
        try_remove(airtable, {record_id});
        throw;
    }
    try_remove(airtable, {record_id});
}

TEST_CASE("orm table: select enums and rating round trip", "[crud][orm-table]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("OrmTable", "Selects");
    auto model = PrimaryModel{
        .primary_key = pk,
        .single_select = PrimarySingleSelectOption::Choice1,
        .multiple_select =
            std::vector<PrimaryMultipleSelectOption>{PrimaryMultipleSelectOption::Option1,
                                                     PrimaryMultipleSelectOption::Option2},
        .rating = 3,
    };
    auto created = airtable.primary().create_one(model);
    const auto record_id = *created.id;
    try {
        REQUIRE(created.single_select == PrimarySingleSelectOption::Choice1);
        REQUIRE(created.multiple_select ==
                std::vector<PrimaryMultipleSelectOption>{PrimaryMultipleSelectOption::Option1,
                                                         PrimaryMultipleSelectOption::Option2});
        REQUIRE(*created.rating == 3);

        auto fetched = airtable.primary().get_one(record_id);
        REQUIRE(fetched.single_select == PrimarySingleSelectOption::Choice1);

        fetched.single_select = PrimarySingleSelectOption::Choice2;
        fetched.multiple_select =
            std::vector<PrimaryMultipleSelectOption>{PrimaryMultipleSelectOption::Option3};
        fetched.rating = 5;
        auto updated = airtable.primary().update_one(fetched);
        REQUIRE(updated.single_select == PrimarySingleSelectOption::Choice2);
        REQUIRE(updated.multiple_select ==
                std::vector<PrimaryMultipleSelectOption>{PrimaryMultipleSelectOption::Option3});
        REQUIRE(*updated.rating == 5);
    } catch (...) {
        try_remove(airtable, {record_id});
        throw;
    }
    try_remove(airtable, {record_id});
}

TEST_CASE("orm table: get with query returns filtered record set", "[crud][orm-table]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("OrmTable", "List");
    std::vector<PrimaryModel> models;
    for (int i = 1; i <= 3; ++i) {
        models.push_back(PrimaryModel{.primary_key = suite + " " + std::to_string(i)});
    }
    auto created = airtable.primary().create_many(models);
    std::vector<std::string> created_ids;
    for (const auto& m : created) {
        created_ids.push_back(*m.id);
    }
    try {
        REQUIRE(created.size() == 3);
        auto results = airtable.primary().get_many(
            AirtableQuery{.formula = "FIND(\"" + suite + "\", {Primary Key})"});
        REQUIRE(results.size() == 3);
        std::set<std::string> expected(created_ids.begin(), created_ids.end());
        std::set<std::string> actual;
        for (const auto& m : results) {
            actual.insert(*m.id);
        }
        REQUIRE(actual == expected);
    } catch (...) {
        try_remove(airtable, created_ids);
        throw;
    }
    try_remove(airtable, created_ids);
}

TEST_CASE("orm table: field selection and max records apply", "[crud][orm-table]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("OrmTable", "Select");
    std::vector<PrimaryModel> models;
    for (int i = 1; i <= 3; ++i) {
        models.push_back(PrimaryModel{.primary_key = suite + " " + std::to_string(i),
                                      .single_line_text = "text " + std::to_string(i)});
    }
    auto created = airtable.primary().create_many(models);
    std::vector<std::string> created_ids;
    for (const auto& m : created) {
        created_ids.push_back(*m.id);
    }
    try {
        auto results = airtable.primary().get_many(
            AirtableQuery{.formula = "FIND(\"" + suite + "\", {Primary Key})",
                          .fields = {std::string(PrimaryFields::kPrimaryKeyId)},
                          .max_records = 2});
        REQUIRE(results.size() == 2); // maxRecords caps the result set
        for (const auto& model : results) {
            REQUIRE(model.primary_key.has_value());
            REQUIRE(model.single_line_text == std::nullopt); // unselected fields decode as null
        }
    } catch (...) {
        try_remove(airtable, created_ids);
        throw;
    }
    try_remove(airtable, created_ids);
}

TEST_CASE("orm table: invalid record id throws", "[crud][orm-table]") {
    auto airtable = make_airtable();
    REQUIRE_THROWS_AS(airtable.primary().get_one("recDOESNOTEXIST123"), AirtableException);
}

TEST_CASE("orm table: batch create/update/delete chunks above the batch limit",
          "[crud][orm-table]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("OrmTable", "Batch");
    constexpr int kCount = 111;
    std::vector<PrimaryModel> models;
    for (int i = 1; i <= kCount; ++i) {
        models.push_back(PrimaryModel{.primary_key = suite + " " + std::to_string(i)});
    }
    std::vector<std::string> created_ids;
    try {
        auto created = airtable.primary().create_many(models);
        for (const auto& m : created) {
            created_ids.push_back(*m.id);
        }
        REQUIRE(created.size() == kCount);
        for (int i = 0; i < kCount; ++i) {
            REQUIRE(created[i].primary_key == suite + " " + std::to_string(i + 1));
        }

        for (auto& m : created) {
            m.primary_key = *m.primary_key + " Updated";
        }
        auto updated = airtable.primary().update_many(created);
        REQUIRE(updated.size() == kCount);
        for (int i = 0; i < kCount; ++i) {
            REQUIRE(updated[i].primary_key == suite + " " + std::to_string(i + 1) + " Updated");
        }

        airtable.primary().delete_many(created_ids);
        REQUIRE_THROWS_AS(airtable.primary().get_one(created_ids.front()), AirtableException);
    } catch (...) {
        try_remove(airtable, created_ids);
        try_sweep_by_prefix(airtable, suite);
        throw;
    }
}

TEST_CASE("orm table: upsert inserts when no match then updates on match", "[crud][orm-table]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("OrmTable", "Upsert");
    std::vector<std::string> created_ids;
    try {
        auto first = airtable.primary().upsert(PrimaryModel{.primary_key = pk, .number_int = 1.0},
                                               {std::string(PrimaryFields::kPrimaryKeyId)});
        REQUIRE(first.was_created);
        created_ids.push_back(*first.model.id);
        REQUIRE(first.model.number_int == 1.0);

        auto second = airtable.primary().upsert(PrimaryModel{.primary_key = pk, .number_int = 2.0},
                                                {std::string(PrimaryFields::kPrimaryKeyId)});
        REQUIRE_FALSE(second.was_created);
        REQUIRE(second.model.id == first.model.id);
        REQUIRE(second.model.number_int == 2.0);
    } catch (...) {
        try_remove(airtable, created_ids);
        try_sweep_by_prefix(airtable, pk);
        throw;
    }
    try_remove(airtable, created_ids);
}

// Duplicate — parity with the rust/go/csharp/python/typescript duplicate suites.
TEST_CASE("orm table: duplicate copies writable fields and recomputes the rest",
          "[crud][orm-table][duplicate]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("Duplicate", "Source");
    auto source = airtable.primary().create_one(PrimaryModel{
        .primary_key = pk,
        .single_line_text = "copy me",
        // Stays <= 10 and != 20 on purpose: filter-by-formula asserts exact counts for
        // `numberInt = 20` and `AND(numberInt > 10, checkbox = true)` on the shared base.
        .number_int = 7.0,
        .rating = 3,
    });
    const auto source_id = *source.id;
    std::string copy_id;
    try {
        auto copy = airtable.primary().duplicate_one(source);
        copy_id = *copy.id;

        REQUIRE(copy_id != source_id);
        REQUIRE(*copy.primary_key == pk);
        REQUIRE(*copy.single_line_text == "copy me");
        REQUIRE(*copy.number_int == 7.0);
        REQUIRE(*copy.rating == 3);
        // Formula (ID) resolves to RECORD_ID(), so on a true copy it is the COPY's id --
        // proof the computed fields were recalculated rather than copied.
        REQUIRE(copy.formula_id->value() == copy_id);
        REQUIRE(copy.auto_number->value() != source.auto_number->value());
        // The source model is untouched.
        REQUIRE(*source.id == source_id);
    } catch (...) {
        try_remove(airtable, {source_id, copy_id});
        throw;
    }
    try_remove(airtable, {source_id, copy_id});
}

TEST_CASE("orm table: duplicate by ids preserves input order", "[crud][orm-table][duplicate]") {
    auto airtable = make_airtable();
    const auto a_pk = primary_key("Duplicate", "OrderA");
    const auto b_pk = primary_key("Duplicate", "OrderB");
    auto a = airtable.primary().create_one(PrimaryModel{.primary_key = a_pk});
    auto b = airtable.primary().create_one(PrimaryModel{.primary_key = b_pk});
    std::vector<std::string> cleanup{*a.id, *b.id};
    try {
        auto copies = airtable.primary().duplicate_many_by_ids({*b.id, *a.id});
        for (const auto& copy : copies) {
            cleanup.push_back(*copy.id);
        }
        REQUIRE(copies.size() == 2);
        REQUIRE(*copies[0].primary_key == b_pk);
        REQUIRE(*copies[1].primary_key == a_pk);
    } catch (...) {
        try_remove(airtable, cleanup);
        throw;
    }
    try_remove(airtable, cleanup);
}
