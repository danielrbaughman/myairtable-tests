// Complex field-type CRUD against the live base: attachments (URL-based, async
// processing), collaborators, computed fields, and the read-only button
// (C# TestComplexProperties parity, 5 cases). Linked records: test_linked_records.cpp.
#include <catch2/catch_test_macros.hpp>

#include <chrono>
#include <thread>

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

TEST_CASE("complex: attachment field round trips via url", "[crud][complex]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("Complex", "Attach");
    const std::string url =
        "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png";

    auto created = airtable.primary().create_one(PrimaryModel{
        .primary_key = pk,
        .attachment = std::vector<AirtableAttachment>{AirtableAttachment{.url = url}},
    });
    const auto record_id = *created.id;
    try {
        // Airtable processes attachments asynchronously — the fresh record may
        // carry just the URL, or be fully populated once processing completes.
        REQUIRE(created.attachment.has_value());
        REQUIRE(created.attachment->size() == 1);

        // Poll until Airtable has enriched the attachment (id populated).
        auto current = created;
        for (int i = 0; i < 10; ++i) {
            if (current.attachment.has_value() && !current.attachment->empty() &&
                current.attachment->front().id.has_value() &&
                !current.attachment->front().id->empty()) {
                break;
            }
            std::this_thread::sleep_for(std::chrono::seconds(2));
            current = airtable.primary().get_one(record_id);
        }
        REQUIRE(current.attachment.has_value());
        REQUIRE(current.attachment->size() == 1);
        REQUIRE(current.attachment->front().url.has_value());
        REQUIRE_FALSE(current.attachment->front().url->empty());
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}

TEST_CASE("complex: user and multi-user fields round trip", "[crud][complex]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("Complex", "Users");
    // Matches the user ID used by the other targets' tests in the shared base.
    const std::string user_id = "usrnZ4k98m0Ipji4e";

    auto created = airtable.primary().create_one(PrimaryModel{
        .primary_key = pk,
        .user = AirtableCollaborator{.id = user_id},
        .user_allow_multiple =
            std::vector<AirtableCollaborator>{AirtableCollaborator{.id = user_id}},
    });
    const auto record_id = *created.id;
    try {
        REQUIRE(created.user.has_value());
        REQUIRE(created.user->id == user_id);
        REQUIRE(created.user_allow_multiple.has_value());
        REQUIRE(created.user_allow_multiple->size() == 1);
        REQUIRE(created.user_allow_multiple->front().id == user_id);

        auto fetched = airtable.primary().get_one(record_id);
        REQUIRE(fetched.user.has_value());
        REQUIRE(fetched.user->id == user_id);
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}

TEST_CASE("complex: computed fields populate on create and read back", "[crud][complex]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("Complex", "Computed");
    auto created = airtable.primary().create_one(PrimaryModel{
        .primary_key = pk,
        .number_float = 5.0,
        .number_int = 10.0,
    });
    const auto record_id = *created.id;
    try {
        // Server-owned: auto number assigned (value variant); created time populated.
        REQUIRE(created.auto_number.has_value());
        REQUIRE(created.auto_number->is_value());
        REQUIRE(created.created_time.has_value());
        // The dedicated createdTime FIELD decodes as a populated value.
        REQUIRE(created.created_at_time.has_value());
        REQUIRE(created.created_at_time->is_value());
        // Formula(ID) reflects the record id.
        REQUIRE(created.formula_id->value() == record_id);
        // Formula(Simple) = numberInt + numberFloat = 15.
        REQUIRE(created.formula_simple->value() == 15.0);
        // Created-by is populated with the caller's user info.
        REQUIRE(created.created_by.has_value());
        REQUIRE(created.created_by->value().has_value());

        auto fetched = airtable.primary().get_one(record_id);
        REQUIRE(fetched.formula_id->value() == record_id);
        REQUIRE(fetched.formula_simple->value() == 15.0);
        REQUIRE(fetched.auto_number == created.auto_number);
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}

TEST_CASE("complex: duration round trips as numeric seconds", "[crud][complex]") {
    // Magnitude assertion: the duration wire unit is seconds, so a 90-minute
    // duration round-trips exactly (not truncated/scaled).
    auto airtable = make_airtable();
    const auto pk = primary_key("Complex", "Duration");
    const Duration duration{90.0 * 60.0};
    auto created = airtable.primary().create_one(PrimaryModel{
        .primary_key = pk,
        .duration = duration,
    });
    const auto record_id = *created.id;
    try {
        REQUIRE(created.duration == duration);
        auto fetched = airtable.primary().get_one(record_id);
        REQUIRE(fetched.duration == duration);
        REQUIRE(fetched.duration->seconds == 5400.0);
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}

TEST_CASE("complex: button field decodes on read", "[crud][complex]") {
    auto airtable = make_airtable();
    const auto pk = primary_key("Complex", "Button");
    auto created = airtable.primary().create_one(PrimaryModel{.primary_key = pk});
    const auto record_id = *created.id;
    try {
        // The button field decodes (either populated or absent) without error.
        auto fetched = airtable.primary().get_one(record_id);
        REQUIRE(fetched.id == record_id);
        if (fetched.button.has_value() && fetched.button->value().has_value()) {
            const auto& button = *fetched.button->value();
            REQUIRE((button.label.has_value() || button.url.has_value()));
        }
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}
