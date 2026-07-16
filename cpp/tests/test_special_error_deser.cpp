// Offline decoding of Airtable's {"specialValue": ...} / {"error": ...}
// computed-field shapes through the generated module, plus a generated-model
// decode proving erroring/special formula + lookup/rollup fields don't break
// deserialization (C# TestSpecialErrorDeser parity, 5 cases).
#include <catch2/catch_test_macros.hpp>

#include <string>
#include <vector>

#include "test_setup.hpp"

using namespace myairtable;

namespace {

template <typename T> T decode(const std::string& raw) {
    return json::parse(raw).get<T>();
}

} // namespace

TEST_CASE("special error deser: special number accepts object form rejects array form",
          "[json][special-error-deser]") {
    REQUIRE(decode<SpecialNumber>(R"({"specialValue":"NaN"})").special_value == "NaN");
    REQUIRE_THROWS(decode<SpecialNumber>(R"(["NaN"])"));
}

TEST_CASE("special error deser: error value accepts object form rejects array form",
          "[json][special-error-deser]") {
    REQUIRE(decode<ErrorValue>(R"({"error":"#ERROR!"})").error == "#ERROR!");
    REQUIRE_THROWS(decode<ErrorValue>(R"(["#ERROR!"])"));
}

TEST_CASE("special error deser: maybe-special-or-error string parses plain string rejects "
          "lookup array",
          "[json][special-error-deser]") {
    const auto plain = decode<MaybeSpecialOrError<std::string>>(R"("Stukenholtz")");
    REQUIRE(plain.value() == "Stukenholtz");
    // Pre-fix regression in other targets: ["Stukenholtz"] accidentally
    // succeeded as Special.
    REQUIRE_THROWS(decode<MaybeSpecialOrError<std::string>>(R"(["Stukenholtz"])"));
}

TEST_CASE("special error deser: vec-or-value lookup shapes", "[json][special-error-deser]") {
    const auto single = decode<VecOrValue<MaybeSpecialOrError<std::string>>>(R"("hello")");
    REQUIRE(single.is_single());
    REQUIRE(single.value()->value() == "hello");

    const auto multiple =
        decode<VecOrValue<MaybeSpecialOrError<std::string>>>(R"(["Stukenholtz Laboratory Inc"])");
    REQUIRE(multiple.clean_values() == std::vector<std::string>{"Stukenholtz Laboratory Inc"});

    const auto with_nulls =
        decode<VecOrValue<MaybeSpecialOrError<std::string>>>(R"(["a", null, "b"])");
    REQUIRE(with_nulls.is_multiple());
    REQUIRE(with_nulls.values().size() == 3);
    REQUIRE(with_nulls.values()[1] == std::nullopt);

    const auto special =
        decode<VecOrValue<MaybeSpecialOrError<double>>>(R"({"specialValue":"NaN"})");
    REQUIRE(special.is_single());
    REQUIRE(special.value()->is_special());
    REQUIRE(special.value()->special()->special_value == "NaN");

    const auto error = decode<VecOrValue<MaybeSpecialOrError<double>>>(R"({"error":"#ERROR!"})");
    REQUIRE(error.is_single());
    REQUIRE(error.value()->is_error());
    REQUIRE(error.value()->error()->error == "#ERROR!");
}

TEST_CASE("special error deser: model decodes erroring computed fields",
          "[json][special-error-deser]") {
    // The exact failure mode that used to throw in other targets: Airtable
    // returns error/special objects for computed fields, while lookup returns
    // an array with nulls.
    const json fields = {
        {std::string(PrimaryFields::kPrimaryKeyId), "ErrDeser"},
        {std::string(PrimaryFields::kFormulaSimpleId), {{"error", "#ERROR!"}}},
        {std::string(PrimaryFields::kFormulaIdId), "recERRDESER123456"},
        {std::string(PrimaryFields::kLookupId), {"Lab A", nullptr}},
        {std::string(PrimaryFields::kRollupId), {{"specialValue", "NaN"}}},
    };
    const json record = {{"fields", fields}};
    const auto model = record.get<PrimaryModel>();

    REQUIRE(model.primary_key == "ErrDeser");
    // Error variant — the variant type IS the assertion.
    REQUIRE(model.formula_simple.has_value());
    REQUIRE(model.formula_simple->is_error());
    REQUIRE(model.formula_id.has_value());
    REQUIRE(model.formula_id->value() == "recERRDESER123456");
    REQUIRE(model.lookup.has_value());
    REQUIRE(model.lookup->clean_values() == std::vector<std::string>{"Lab A"});
    REQUIRE(model.lookup->is_multiple());
    REQUIRE(model.lookup->values().size() == 2);
    REQUIRE(model.rollup.has_value());
    REQUIRE(model.rollup->is_single());
    REQUIRE(model.rollup->value()->is_special());
    REQUIRE(model.rollup->value()->special()->special_value == "NaN");
}
