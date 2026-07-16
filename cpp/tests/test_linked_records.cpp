// Linked records as raw record-ID lists; resolution via the linked table's
// get() (C# TestLinkedRecords parity, 4 cases).
#include <catch2/catch_test_macros.hpp>

#include <algorithm>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {
void try_remove_primary(Airtable& airtable, const std::optional<std::string>& id) {
    if (!id.has_value() || id->empty()) {
        return;
    }
    try {
        airtable.primary().remove(*id);
    } catch (const AirtableException&) {
    }
}

void try_remove_secondaries(Airtable& airtable,
                            const std::vector<std::optional<std::string>>& ids) {
    for (const auto& id : ids) {
        if (!id.has_value() || id->empty()) {
            continue;
        }
        try {
            airtable.secondary().remove(*id);
        } catch (const AirtableException&) {
        }
    }
}
} // namespace

TEST_CASE("linked: record fields round trip via record ids", "[crud][linked]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Linked", "RoundTrip");
    std::optional<std::string> sec1_id, sec2_id, prim_id;
    try {
        sec1_id = *airtable.secondary().create(SecondaryModel{.name = suite + " S1"}).id;
        sec2_id = *airtable.secondary().create(SecondaryModel{.name = suite + " S2"}).id;

        auto prim = airtable.primary().create(PrimaryModel{
            .primary_key = suite,
            .link_multiple = std::vector<std::string>{*sec1_id, *sec2_id},
            .link_single = std::vector<std::string>{*sec1_id},
        });
        prim_id = prim.id;
        REQUIRE(prim.link_single == std::vector<std::string>{*sec1_id});
        REQUIRE(prim.link_multiple == std::vector<std::string>{*sec1_id, *sec2_id});

        auto fetched = airtable.primary().get(*prim_id);
        REQUIRE(fetched.link_single == std::vector<std::string>{*sec1_id});
        REQUIRE(fetched.link_multiple == std::vector<std::string>{*sec1_id, *sec2_id});

        fetched.link_single = std::vector<std::string>{*sec2_id};
        fetched.link_multiple = std::vector<std::string>{*sec1_id};
        auto updated = airtable.primary().update(fetched);
        REQUIRE(updated.link_single == std::vector<std::string>{*sec2_id});
        REQUIRE(updated.link_multiple == std::vector<std::string>{*sec1_id});
    } catch (...) {
        try_remove_primary(airtable, prim_id);
        try_remove_secondaries(airtable, {sec1_id, sec2_id});
        throw;
    }
    try_remove_primary(airtable, prim_id);
    try_remove_secondaries(airtable, {sec1_id, sec2_id});
}

TEST_CASE("linked: single record resolves via secondary get", "[crud][linked]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Linked", "Single");
    std::optional<std::string> sec_id, prim_id;
    try {
        sec_id = *airtable.secondary()
                      .create(SecondaryModel{.name = suite + " Target", .value = "sv"})
                      .id;

        auto prim = airtable.primary().create(PrimaryModel{
            .primary_key = suite,
            .link_single = std::vector<std::string>{*sec_id},
        });
        prim_id = prim.id;
        REQUIRE(prim.link_single->size() == 1);
        const auto linked_id = prim.link_single->front();
        REQUIRE(linked_id == *sec_id);

        auto linked = airtable.secondary().get(linked_id);
        REQUIRE(linked.name == suite + " Target");
        REQUIRE(linked.value == "sv");
    } catch (...) {
        try_remove_primary(airtable, prim_id);
        try_remove_secondaries(airtable, {sec_id});
        throw;
    }
    try_remove_primary(airtable, prim_id);
    try_remove_secondaries(airtable, {sec_id});
}

TEST_CASE("linked: empty link-single decodes as nullopt", "[crud][linked]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Linked", "FetchNil");
    auto prim = airtable.primary().create(PrimaryModel{.primary_key = suite + " No Links"});
    const auto prim_id = prim.id;
    try {
        // Airtable omits empty link fields entirely; link_single decodes as nullopt/empty.
        REQUIRE((!prim.link_single.has_value() || prim.link_single->empty()));
    } catch (...) {
        try_remove_primary(airtable, prim_id);
        throw;
    }
    try_remove_primary(airtable, prim_id);
}

TEST_CASE("linked: multi records resolve via secondary get", "[crud][linked]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Linked", "Multi");
    std::optional<std::string> sec1_id, sec2_id, prim_id;
    try {
        sec1_id = *airtable.secondary().create(SecondaryModel{.name = suite + " T1"}).id;
        sec2_id = *airtable.secondary().create(SecondaryModel{.name = suite + " T2"}).id;

        auto prim = airtable.primary().create(PrimaryModel{
            .primary_key = suite,
            .link_multiple = std::vector<std::string>{*sec1_id, *sec2_id},
        });
        prim_id = prim.id;

        auto linked =
            airtable.secondary().get(prim.link_multiple.value_or(std::vector<std::string>{}));
        REQUIRE(linked.size() == 2);
        std::vector<std::string> names;
        for (const auto& model : linked) {
            names.push_back(model.name.value_or(""));
        }
        REQUIRE(std::find(names.begin(), names.end(), suite + " T1") != names.end());
        REQUIRE(std::find(names.begin(), names.end(), suite + " T2") != names.end());
    } catch (...) {
        try_remove_primary(airtable, prim_id);
        try_remove_secondaries(airtable, {sec1_id, sec2_id});
        throw;
    }
    try_remove_primary(airtable, prim_id);
    try_remove_secondaries(airtable, {sec1_id, sec2_id});
}
