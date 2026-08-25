// Offline (no-network) model serialization parity (C# TestSerializing, 16
// cases). Decodes a record's `fields` object (keyed by field id) into the
// generated PrimaryModel, and exercises the writable-only payload builders +
// snapshot/dirty machinery. Network-bound cases live in
// test_serializing_integration.cpp.
#include <catch2/catch_test_macros.hpp>

#include <set>
#include <string>
#include <vector>

#include "test_setup.hpp"

using namespace myairtable;

namespace {

std::string fid(std::string_view id) {
    return std::string(id);
}

/// Decode a fields object (keyed by field id) into a PrimaryModel — the C#
/// `fields.Deserialize<PrimaryModel>()` analog (the C++ decoder takes the
/// {id, createdTime, fields} envelope).
PrimaryModel decode(const json& fields) {
    const json record = {{"fields", fields}};
    return record.get<PrimaryModel>();
}

std::set<std::string> keys_of(const json& object) {
    std::set<std::string> keys;
    for (const auto& [key, value] : object.items()) {
        keys.insert(key);
    }
    return keys;
}

} // namespace

// ---- decode ----

TEST_CASE("serializing: decode primary fields keyed by field id", "[json][serializing]") {
    const auto model = decode(json{
        {fid(PrimaryFields::kPrimaryKeyId), "PK Value"},
        {fid(PrimaryFields::kSingleLineTextId), "hello"},
        {fid(PrimaryFields::kNumberIntId), 42},
        {fid(PrimaryFields::kCheckboxId), true},
    });
    REQUIRE(model.primary_key == "PK Value");
    REQUIRE(model.single_line_text == "hello");
    REQUIRE(model.number_int == 42.0);
    REQUIRE(model.checkbox == true);
}

TEST_CASE("serializing: date-only and datetime field values both decode", "[json][serializing]") {
    const auto model = decode(json{
        {fid(PrimaryFields::kDateId), "2024-01-15"},
        {fid(PrimaryFields::kDateWithTimeId), "2024-01-15T10:30:00.000Z"},
    });
    REQUIRE(model.date == try_parse_datetime("2024-01-15T00:00:00.000Z"));
    REQUIRE(model.date_with_time == try_parse_datetime("2024-01-15T10:30:00.000Z"));
}

TEST_CASE("serializing: missing fields are null", "[json][serializing]") {
    const auto model = decode(json{{fid(PrimaryFields::kPrimaryKeyId), "only pk"}});
    REQUIRE(model.primary_key == "only pk");
    REQUIRE(model.single_line_text == std::nullopt);
    REQUIRE(model.number_int == std::nullopt);
    REQUIRE(model.date == std::nullopt);
}

TEST_CASE("serializing: empty fields object decodes cleanly", "[json][serializing]") {
    const auto model = decode(json::object());
    REQUIRE(model.primary_key == std::nullopt);
    REQUIRE(model.is_new());
}

// ---- encode / payload builders ----

TEST_CASE("serializing: encode produces fields keyed by field id", "[json][serializing]") {
    const auto model = PrimaryModel{.number_int = 7.0, .primary_key = "x"};
    const auto record = model.to_create_fields();
    REQUIRE(keys_of(record) == std::set<std::string>{fid(PrimaryFields::kPrimaryKeyId),
                                                     fid(PrimaryFields::kNumberIntId)});
    REQUIRE(record.at(fid(PrimaryFields::kPrimaryKeyId)).get<std::string>() == "x");
}

TEST_CASE("serializing: to_record emits field values keyed by field id", "[json][serializing]") {
    const auto model = PrimaryModel{.email = "a@b.c", .primary_key = "rec"};
    const auto record = model.to_record();
    REQUIRE(record.at(fid(PrimaryFields::kPrimaryKeyId)).get<std::string>() == "rec");
    REQUIRE(record.at(fid(PrimaryFields::kEmailId)).get<std::string>() == "a@b.c");
}

TEST_CASE("serializing: round trip preserves all writable field values", "[json][serializing]") {
    const auto original = PrimaryModel{
        .checkbox = true,
        .number_float = 2.5,
        .number_int = 5.0,
        .primary_key = "rt",
        .single_line_text = "text",
    };
    const json encoded = original.to_record();
    const auto decoded = decode(encoded);
    REQUIRE(decoded.primary_key == original.primary_key);
    REQUIRE(decoded.single_line_text == original.single_line_text);
    REQUIRE(decoded.checkbox == original.checkbox);
    REQUIRE(decoded.number_int == original.number_int);
    REQUIRE(decoded.number_float == original.number_float);
}

TEST_CASE("serializing: fresh model omits null fields (sparse write)", "[json][serializing]") {
    const auto model = PrimaryModel{.primary_key = "sparse"};
    const auto record = model.to_create_fields();
    REQUIRE(keys_of(record) == std::set<std::string>{fid(PrimaryFields::kPrimaryKeyId)});
    // The full-record encoding also skips nulls, incl. the computed members.
    const json encoded = model.to_record();
    REQUIRE(encoded == json{{fid(PrimaryFields::kPrimaryKeyId), "sparse"}});
}

TEST_CASE("serializing: rating is a typed integer", "[json][serializing]") {
    // `rating` maps to GenericType.INTEGER, so it is a real int64_t rather than the
    // type-erased json it used to be. It still serializes as a JSON number.
    const auto model = PrimaryModel{.primary_key = "r", .rating = 3};
    REQUIRE(*model.rating == 3);
    REQUIRE(model.to_create_fields().at(fid(PrimaryFields::kRatingId)).get<int>() == 3);
}

// ---- snapshot / dirty tracking ----

TEST_CASE("serializing: fresh decode has no dirty fields after snapshot", "[json][serializing]") {
    auto model = decode(json{{fid(PrimaryFields::kPrimaryKeyId), "clean"}});
    model.take_snapshot();
    REQUIRE(model.dirty_fields().empty());
}

TEST_CASE("serializing: mutating a writable field marks only that field dirty",
          "[json][serializing]") {
    auto model = decode(json{
        {fid(PrimaryFields::kPrimaryKeyId), "pk"},
        {fid(PrimaryFields::kSingleLineTextId), "before"},
    });
    model.take_snapshot();
    model.single_line_text = "after";
    REQUIRE(keys_of(model.dirty_fields()) ==
            std::set<std::string>{fid(PrimaryFields::kSingleLineTextId)});
}

TEST_CASE("serializing: take_snapshot clears the dirty set", "[json][serializing]") {
    auto model = decode(json{{fid(PrimaryFields::kPrimaryKeyId), "pk"}});
    model.take_snapshot();
    model.single_line_text = "mutated";
    REQUIRE_FALSE(model.dirty_fields().empty());
    model.take_snapshot();
    REQUIRE(model.dirty_fields().empty());
}

TEST_CASE("serializing: cleared writable field becomes json null", "[json][serializing]") {
    auto model = decode(json{
        {fid(PrimaryFields::kPrimaryKeyId), "pk"},
        {fid(PrimaryFields::kNumberIntId), 9.5},
    });
    model.take_snapshot();
    model.number_int = std::nullopt;
    const auto dirty = model.dirty_fields();
    REQUIRE(dirty.contains(fid(PrimaryFields::kNumberIntId)));
    REQUIRE(dirty.at(fid(PrimaryFields::kNumberIntId)).is_null()); // cleared = JSON null
}

TEST_CASE("serializing: shrinking a multi-value field is detected as dirty",
          "[json][serializing]") {
    // Regression: dirty tracking must use structural equality, not
    // formula-style coercion (which compares arrays by their first element
    // only and would miss [a, b] -> [a]).
    auto model =
        PrimaryModel{.id = "rec1", .link_multiple = std::vector<std::string>{"recA", "recB"}};
    model.take_snapshot();
    model.link_multiple = std::vector<std::string>{"recA"};
    const auto dirty = model.dirty_fields();
    REQUIRE(dirty.contains(fid(PrimaryFields::kLinkMultipleId)));
    const auto& shrunk = dirty.at(fid(PrimaryFields::kLinkMultipleId));
    REQUIRE(shrunk.is_array());
    REQUIRE(shrunk.size() == 1);
    REQUIRE(shrunk[0].get<std::string>() == "recA");
}

// ---- identity ----

TEST_CASE("serializing: unsaved model reports is_new true", "[json][serializing]") {
    const auto model = PrimaryModel{.primary_key = "new"};
    REQUIRE(model.is_new());
    REQUIRE(model.id == std::nullopt);
}

TEST_CASE("serializing: model with id reports is_new false", "[json][serializing]") {
    const auto model = PrimaryModel{.id = "recXYZ", .primary_key = "saved"};
    REQUIRE_FALSE(model.is_new());
}
