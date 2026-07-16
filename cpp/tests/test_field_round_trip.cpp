// Field-type round-trip completeness via re-fetch (C# TestFieldRoundTrip
// parity, 5 cases). Several field types were only asserted on the create
// response (or only offline-decoded), never written and read back through the
// live API, and clearing/removing multi-value fields was untested. Each case
// creates, optionally updates, re-fetches, and asserts the server-side value.
#include <catch2/catch_test_macros.hpp>

#include <string>
#include <vector>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {

// Shared-base user, as in the complex-properties suite.
constexpr const char* kUserId = "usrnZ4k98m0Ipji4e";

DateTime utc(const std::string& text) {
    return *try_parse_datetime(text);
}

void try_remove(Airtable& airtable, const std::string& id) {
    if (id.empty()) {
        return;
    }
    try {
        airtable.primary().remove(id);
    } catch (const AirtableException&) {
    }
}

} // namespace

TEST_CASE("field round trip: date with time writes and reads back", "[crud][field-round-trip]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("FieldRT", "DateTime");
    const auto dt = utc("2024-03-15T14:30:00.000Z");
    auto created =
        airtable.primary().create(PrimaryModel{.date_with_time = dt, .primary_key = suite});
    const auto record_id = *created.id;
    try {
        auto fetched = airtable.primary().get(record_id);
        REQUIRE(fetched.date_with_time == dt);
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}

TEST_CASE("field round trip: rich text and percent/currency read back",
          "[crud][field-round-trip]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("FieldRT", "Rich");
    auto created = airtable.primary().create(PrimaryModel{
        .currency_float = 19.99,
        .currency_int = 100.0,
        .long_text_with_rich_text = "**bold** and _italic_ text",
        .percent_float = 0.333,
        .percent_int = 0.5,
        .primary_key = suite,
    });
    const auto record_id = *created.id;
    try {
        auto fetched = airtable.primary().get(record_id);
        REQUIRE(fetched.long_text_with_rich_text == "**bold** and _italic_ text");
        REQUIRE(fetched.percent_int == 0.5);
        REQUIRE(fetched.percent_float == 0.333);
        REQUIRE(fetched.currency_int == 100.0);
        REQUIRE(fetched.currency_float == 19.99);
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}

TEST_CASE("field round trip: clearing single and multi select reads back empty",
          "[crud][field-round-trip]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("FieldRT", "ClearSelect");
    auto created = airtable.primary().create(PrimaryModel{
        .multiple_select =
            std::vector<PrimaryMultipleSelectOption>{PrimaryMultipleSelectOption::Option1,
                                                     PrimaryMultipleSelectOption::Option2},
        .primary_key = suite,
        .single_select = PrimarySingleSelectOption::Choice1,
    });
    const auto record_id = *created.id;
    try {
        REQUIRE(created.single_select == PrimarySingleSelectOption::Choice1);

        created.single_select = std::nullopt;
        created.multiple_select = std::vector<PrimaryMultipleSelectOption>{};
        airtable.primary().update(created);

        auto fetched = airtable.primary().get(record_id);
        REQUIRE(fetched.single_select == std::nullopt);
        REQUIRE((!fetched.multiple_select.has_value() || fetched.multiple_select->empty()));
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}

TEST_CASE("field round trip: removing a collaborator reads back null", "[crud][field-round-trip]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("FieldRT", "RemoveUser");
    auto created = airtable.primary().create(
        PrimaryModel{.primary_key = suite, .user = AirtableCollaborator{.id = kUserId}});
    const auto record_id = *created.id;
    try {
        REQUIRE(created.user.has_value());
        REQUIRE(created.user->id == kUserId);

        created.user = std::nullopt;
        airtable.primary().update(created);

        auto fetched = airtable.primary().get(record_id);
        REQUIRE(fetched.user == std::nullopt);
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}

TEST_CASE("field round trip: attachment replace and remove read back", "[crud][field-round-trip]") {
    constexpr const char* kUrlA =
        "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png";
    constexpr const char* kUrlB = "https://www.w3.org/Icons/w3c_home.png";
    auto airtable = make_airtable();
    const auto suite = primary_key("FieldRT", "Attach");
    auto created = airtable.primary().create(PrimaryModel{
        .attachment = std::vector<AirtableAttachment>{AirtableAttachment{.url = kUrlA}},
        .primary_key = suite,
    });
    const auto record_id = *created.id;
    try {
        // Replace the attachment with a different one.
        created.attachment = std::vector<AirtableAttachment>{AirtableAttachment{.url = kUrlB}};
        airtable.primary().update(created);
        auto replaced = airtable.primary().get(record_id);
        REQUIRE(replaced.attachment.has_value());
        REQUIRE(replaced.attachment->size() == 1);

        // Remove all attachments.
        replaced.attachment = std::vector<AirtableAttachment>{};
        airtable.primary().update(replaced);
        auto cleared = airtable.primary().get(record_id);
        REQUIRE((!cleared.attachment.has_value() || cleared.attachment->empty()));
    } catch (...) {
        try_remove(airtable, record_id);
        throw;
    }
    try_remove(airtable, record_id);
}
