// Filter-by-formula integration tests — the generated {Model}::F filter DSL
// against the live base (C# TestFilterByFormula parity, 18 cases).
#include <catch2/catch_test_macros.hpp>

#include <algorithm>
#include <set>
#include <string>
#include <vector>

#include "dynamic/types/primary_view.hpp"
#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {

/// Scope a formula to specific record IDs so suite runs don't interfere.
std::string scope_to(const std::vector<std::string>& ids) {
    std::vector<std::string> parts;
    parts.reserve(ids.size());
    for (const auto& id : ids) {
        parts.push_back("RECORD_ID()='" + id + "'");
    }
    return parts.size() == 1 ? parts.front() : Formulas::or_(parts);
}

Fields primary_fields_for(const std::string& name) {
    Fields fields({}, PrimaryFields::kNameToId);
    fields.set_string(PrimaryFields::kPrimaryKeyId, name);
    return fields;
}

std::vector<std::string> names_of(const std::vector<DictTable::Record>& records) {
    std::vector<std::string> out;
    for (const auto& record : records) {
        if (const auto name = record.fields.get_string(PrimaryFields::kPrimaryKeyId)) {
            out.push_back(*name);
        }
    }
    std::sort(out.begin(), out.end());
    return out;
}

/// Count of Primary dict records matching `formula` within `scope`.
size_t count_matching(Airtable& airtable, const std::string& scope, const std::string& formula) {
    return airtable.primary()
        .dict()
        .get_many(AirtableQuery{.formula = Formulas::and_({scope, formula})})
        .size();
}

std::vector<std::string> match_names(Airtable& airtable, const std::string& scope,
                                     const std::string& formula) {
    return names_of(airtable.primary().dict().get_many(
        AirtableQuery{.formula = Formulas::and_({scope, formula})}));
}

bool contains_id(const std::vector<PrimaryModel>& models, const std::string& id) {
    return std::any_of(models.begin(), models.end(),
                       [&](const PrimaryModel& m) { return m.id == id; });
}

std::vector<std::string> ids_of(const std::vector<DictTable::Record>& records) {
    std::vector<std::string> ids;
    ids.reserve(records.size());
    for (const auto& record : records) {
        ids.push_back(record.id);
    }
    return ids;
}

// ---- cleanup helpers (best effort) ------------------------------------------

void try_remove(Airtable& airtable, const std::vector<std::string>& ids) {
    if (ids.empty()) {
        return;
    }
    try {
        airtable.primary().dict().delete_many(ids);
    } catch (const AirtableException&) {
    }
}

void try_remove_secondaries(Airtable& airtable, const std::vector<std::string>& ids) {
    try {
        airtable.secondary().dict().delete_many(ids);
    } catch (const AirtableException&) {
    }
}

void try_remove_formulas(Airtable& airtable, const std::vector<std::string>& ids) {
    try {
        airtable.formulas().dict().delete_many(ids);
    } catch (const AirtableException&) {
    }
}

} // namespace

TEST_CASE("filter by formula: text field equals", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "TextEq");
    auto created = airtable.primary().create_one(
        PrimaryModel{.primary_key = suite, .single_line_text = "UniqueFilterValue"});
    const auto record_id = *created.id;
    try {
        const auto filter = PrimaryModel::F.single_line_text.eq("UniqueFilterValue");
        auto results = airtable.primary().get_many(AirtableQuery{.formula = filter});
        REQUIRE(contains_id(results, record_id));
    } catch (...) {
        try_remove(airtable, {record_id});
        throw;
    }
    try_remove(airtable, {record_id});
}

TEST_CASE("filter by formula: record id filter", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "RecordID");
    auto created = airtable.primary().create_one(PrimaryModel{.primary_key = suite});
    const auto record_id = *created.id;
    try {
        auto results =
            airtable.primary().get_many(AirtableQuery{.formula = PrimaryModel::F.id.eq(record_id)});
        REQUIRE(results.size() == 1);
        REQUIRE(results[0].id == record_id);
    } catch (...) {
        try_remove(airtable, {record_id});
        throw;
    }
    try_remove(airtable, {record_id});
}

TEST_CASE("filter by formula: number field filter", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "Number");
    auto c1 = airtable.primary().create_one(
        PrimaryModel{.number_int = 10.0, .primary_key = suite + " 1"});
    auto c2 = airtable.primary().create_one(
        PrimaryModel{.number_int = 20.0, .primary_key = suite + " 2"});
    const std::vector<std::string> ids{*c1.id, *c2.id};
    try {
        auto results = airtable.primary().get_many(
            AirtableQuery{.formula = PrimaryModel::F.number_int.greater_than(15)});
        REQUIRE(contains_id(results, *c2.id));
        REQUIRE_FALSE(contains_id(results, *c1.id));
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("filter by formula: and combinator", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "AND");
    auto created = airtable.primary().create_one(
        PrimaryModel{.checkbox = true, .number_int = 42.0, .primary_key = suite});
    const auto record_id = *created.id;
    try {
        const auto& f = PrimaryModel::F;
        const auto filter = Formulas::and_({f.checkbox.is_true(), f.number_int.eq(42)});
        auto results = airtable.primary().get_many(AirtableQuery{.formula = filter});
        REQUIRE(contains_id(results, record_id));
    } catch (...) {
        try_remove(airtable, {record_id});
        throw;
    }
    try_remove(airtable, {record_id});
}

TEST_CASE("filter by formula: filter by view", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    std::vector<Fields> creates;
    for (int i = 0; i < 5; ++i) {
        creates.push_back(primary_fields_for("Filter Test " + std::to_string(i)));
    }
    for (int i = 0; i < 5; ++i) {
        creates.push_back(primary_fields_for("Don't Include Test " + std::to_string(i)));
    }
    auto created = airtable.primary().dict().create_many(creates);
    const auto ids = ids_of(created);
    try {
        auto results = airtable.primary().dict().get_many(
            AirtableQuery{.view = PrimaryView::FilterByView.id()});
        REQUIRE(results.size() >= 5);
        for (const auto& record : results) {
            const auto name = record.fields.get_string(PrimaryFields::kPrimaryKeyId).value_or("");
            REQUIRE(name.rfind("Filter Test", 0) == 0);
        }
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("filter by formula: record id in list", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "InList");
    std::vector<Fields> creates;
    for (int i = 0; i < 3; ++i) {
        creates.push_back(primary_fields_for(suite + " " + std::to_string(i)));
    }
    auto created = airtable.primary().dict().create_many(creates);
    const auto ids = ids_of(created);
    try {
        const auto& f = PrimaryModel::F;
        auto multi = airtable.primary().dict().get_many(
            AirtableQuery{.formula = f.id.in_list({ids[0], ids[1]})});
        REQUIRE(multi.size() == 2);

        auto single =
            airtable.primary().dict().get_many(AirtableQuery{.formula = f.id.in_list({ids[0]})});
        REQUIRE(single.size() == 1);

        auto empty = airtable.primary().dict().get_many(
            AirtableQuery{.formula = f.id.in_list(std::vector<std::string>{})});
        REQUIRE(empty.empty());
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("filter by formula: text field predicates", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    auto created = airtable.primary().dict().create_many(std::vector<Fields>{
        primary_fields_for("FbText Alpha One"),
        primary_fields_for("FbText Alpha Two"),
        primary_fields_for("FbText Beta One"),
    });
    const auto ids = ids_of(created);
    const auto scope = scope_to(ids);
    const auto& f = PrimaryModel::F;
    try {
        REQUIRE(count_matching(airtable, scope, f.primary_key.eq("FbText Alpha One")) == 1);
        REQUIRE(count_matching(airtable, scope, f.primary_key.neq("FbText Alpha One")) == 2);
        REQUIRE(count_matching(airtable, scope, f.primary_key.contains("Alpha", false, true)) == 2);
        REQUIRE(count_matching(airtable, scope,
                               f.primary_key.contains_any({"Alpha", "Beta"}, false, true)) == 3);
        REQUIRE(count_matching(airtable, scope,
                               f.primary_key.contains_all({"Alpha", "One"}, false, true)) == 1);
        REQUIRE(count_matching(airtable, scope, f.primary_key.not_contains("Alpha", false, true)) ==
                1);
        REQUIRE(count_matching(airtable, scope,
                               f.primary_key.starts_with("FbText Alpha", false, true)) == 2);
        REQUIRE(count_matching(airtable, scope,
                               f.primary_key.not_starts_with("FbText Alpha", false, true)) == 1);
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("filter by formula: number field predicates", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "NumPreds");
    const std::vector<int64_t> values{10, 20, 30};
    std::vector<Fields> creates;
    for (size_t i = 0; i < values.size(); ++i) {
        auto fields = primary_fields_for(suite + " " + std::to_string(i));
        fields.set_long(PrimaryFields::kNumberIntId, values[i]);
        creates.push_back(std::move(fields));
    }
    auto created = airtable.primary().dict().create_many(creates);
    const auto ids = ids_of(created);
    const auto scope = scope_to(ids);
    const auto& f = PrimaryModel::F;
    try {
        REQUIRE(count_matching(airtable, scope, f.number_int.eq(20)) == 1);
        REQUIRE(count_matching(airtable, scope, f.number_int.neq(20)) == 2);
        REQUIRE(count_matching(airtable, scope, f.number_int.greater_than(10)) == 2);
        REQUIRE(count_matching(airtable, scope, f.number_int.less_than(30)) == 2);
        REQUIRE(count_matching(airtable, scope, f.number_int.greater_than_or_equals(20)) == 2);
        REQUIRE(count_matching(airtable, scope, f.number_int.less_than_or_equals(20)) == 2);
        REQUIRE(count_matching(airtable, scope, f.number_int.between(10, 30)) == 3);
        REQUIRE(count_matching(airtable, scope, f.number_int.between(10, 30, false)) == 1);
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("filter by formula: boolean field filter", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "Bool");
    auto f1 = primary_fields_for(suite + " A");
    f1.set_bool(PrimaryFields::kCheckboxId, true);
    auto f2 = primary_fields_for(suite + " B");
    f2.set_bool(PrimaryFields::kCheckboxId, false);
    auto created = airtable.primary().dict().create_many(std::vector<Fields>{f1, f2});
    const auto ids = ids_of(created);
    const auto scope = scope_to(ids);
    const auto& f = PrimaryModel::F;
    try {
        REQUIRE(count_matching(airtable, scope, f.checkbox.eq(true)) == 1);
        REQUIRE(count_matching(airtable, scope, f.checkbox.eq(false)) >= 1);
        REQUIRE(count_matching(airtable, scope, f.checkbox.is_true()) == 1);
        REQUIRE(count_matching(airtable, scope, f.checkbox.is_false()) >= 1);
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("filter by formula: attachments field filter", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "Attach");
    auto f1 = primary_fields_for(suite + " A");
    const json attachments =
        json::array({json{{"url", "https://www.google.com/images/branding/googlelogo/2x/"
                                  "googlelogo_color_272x92dp.png"}}});
    f1.set(PrimaryFields::kAttachmentId, attachments);
    auto f2 = primary_fields_for(suite + " B");
    auto created = airtable.primary().dict().create_many(std::vector<Fields>{f1, f2});
    const auto ids = ids_of(created);
    const auto scope = scope_to(ids);
    const auto& f = PrimaryModel::F;
    try {
        auto not_empty = airtable.primary().dict().get_many(
            AirtableQuery{.formula = Formulas::and_({scope, f.attachment.is_not_empty()})});
        REQUIRE_FALSE(not_empty.empty());
        auto empty = airtable.primary().dict().get_many(
            AirtableQuery{.formula = Formulas::and_({scope, f.attachment.is_empty()})});
        REQUIRE_FALSE(empty.empty());
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("filter by formula: date field predicates", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "Date");
    auto f1 = primary_fields_for(suite + " A");
    f1.set_string(PrimaryFields::kDateId, "2024-01-15");
    auto f2 = primary_fields_for(suite + " B");
    f2.set_string(PrimaryFields::kDateId, "2024-06-15");
    auto f3 = primary_fields_for(suite + " C"); // no date
    auto created = airtable.primary().dict().create_many(std::vector<Fields>{f1, f2, f3});
    const auto ids = ids_of(created);
    const auto scope = scope_to(ids);
    const auto& f = PrimaryModel::F;
    try {
        REQUIRE(count_matching(airtable, scope, f.date.on("2024-01-15")) == 1);
        REQUIRE(count_matching(airtable, scope, f.date.not_on("2024-01-15")) >= 1);
        REQUIRE(count_matching(airtable, scope, f.date.on_or_after("2024-06-15")) >= 1);
        REQUIRE(count_matching(airtable, scope, f.date.on_or_before("2024-01-15")) >= 1);
        REQUIRE(count_matching(airtable, scope, f.date.after("2024-01-15")) >= 1);
        REQUIRE(count_matching(airtable, scope, f.date.before("2024-06-15")) >= 1);
        REQUIRE(count_matching(airtable, scope, f.date.between("2024-01-15", "2024-06-15")) == 2);
        REQUIRE(count_matching(airtable, scope,
                               f.date.between("2024-01-01", "2024-12-31", false)) == 2);
        REQUIRE(count_matching(airtable, scope, f.date.is_not_empty()) >= 1);
        REQUIRE(count_matching(airtable, scope, f.date.is_empty()) >= 1);
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("filter by formula: date field chained", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "DateChain");
    auto f1 = primary_fields_for(suite + " A");
    f1.set_string(PrimaryFields::kDateId, "2024-01-15");
    auto f2 = primary_fields_for(suite + " B");
    f2.set_string(PrimaryFields::kDateId, "2024-06-15");
    auto created = airtable.primary().dict().create_many(std::vector<Fields>{f1, f2});
    const auto ids = ids_of(created);
    const auto scope = scope_to(ids);
    const auto& f = PrimaryModel::F;
    try {
        auto past = airtable.primary().dict().get_many(
            AirtableQuery{.formula = Formulas::and_({scope, f.date.before().days_ago(1)})});
        REQUIRE_FALSE(past.empty());
        auto recent = airtable.primary().dict().get_many(
            AirtableQuery{.formula = Formulas::and_({scope, f.date.after().years_ago(100)})});
        REQUIRE_FALSE(recent.empty());
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("filter by formula: complex formulas", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "Complex");
    auto f1 = primary_fields_for(suite + " A");
    f1.set_long(PrimaryFields::kNumberIntId, 10);
    f1.set_bool(PrimaryFields::kCheckboxId, true);
    auto f2 = primary_fields_for(suite + " B");
    f2.set_long(PrimaryFields::kNumberIntId, 20);
    f2.set_bool(PrimaryFields::kCheckboxId, false);
    auto f3 = primary_fields_for(suite + " C");
    f3.set_long(PrimaryFields::kNumberIntId, 30);
    f3.set_bool(PrimaryFields::kCheckboxId, true);
    auto created = airtable.primary().dict().create_many(std::vector<Fields>{f1, f2, f3});
    const auto ids = ids_of(created);
    const auto scope = scope_to(ids);
    const auto& f = PrimaryModel::F;
    try {
        REQUIRE(count_matching(
                    airtable, scope,
                    Formulas::and_({f.number_int.greater_than(10), f.checkbox.is_true()})) == 1);
        REQUIRE(count_matching(airtable, scope,
                               Formulas::and_({f.primary_key.contains(suite, false, true),
                                               f.number_int.greater_than_or_equals(20)})) == 2);
        REQUIRE(count_matching(airtable, scope,
                               Formulas::or_({f.number_int.eq(10), f.number_int.eq(30)})) == 2);
        REQUIRE(count_matching(airtable, scope,
                               Formulas::or_({f.checkbox.is_true(), f.number_int.eq(20)})) == 3);
        REQUIRE(count_matching(airtable, scope,
                               Formulas::xor_({f.checkbox.is_true(),
                                               f.number_int.greater_than_or_equals(30)})) == 1);
        REQUIRE(count_matching(airtable, scope, Formulas::not_(f.primary_key.eq(suite + " A"))) ==
                2);
        REQUIRE(count_matching(
                    airtable, scope,
                    Formulas::and_({Formulas::or_({f.number_int.eq(10), f.number_int.eq(30)}),
                                    f.checkbox.is_true()})) == 2);
        REQUIRE(count_matching(airtable, scope,
                               Formulas::or_({Formulas::and_({f.number_int.greater_than(10),
                                                              f.checkbox.is_true()}),
                                              f.primary_key.eq(suite + " A")})) == 2);
        REQUIRE(count_matching(airtable, scope,
                               Formulas::not_(Formulas::and_(
                                   {f.checkbox.is_true(), f.number_int.greater_than(20)}))) == 2);
        REQUIRE(count_matching(airtable, scope,
                               Formulas::and_({Formulas::not_(f.checkbox.is_true()),
                                               f.number_int.greater_than_or_equals(20)})) == 1);
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("filter by formula: max records", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "MaxRec");
    std::vector<Fields> creates;
    for (int i = 1; i <= 5; ++i) {
        creates.push_back(primary_fields_for(suite + " " + std::to_string(i)));
    }
    auto created = airtable.primary().dict().create_many(creates);
    const auto ids = ids_of(created);
    try {
        auto results = airtable.primary().dict().get_many(
            AirtableQuery{.formula = scope_to(ids), .max_records = 3});
        REQUIRE(results.size() == 3);
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("filter by formula: sort records", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "Sort");
    auto f1 = primary_fields_for(suite + " C");
    f1.set_long(PrimaryFields::kNumberIntId, 30);
    auto f2 = primary_fields_for(suite + " A");
    f2.set_long(PrimaryFields::kNumberIntId, 10);
    auto f3 = primary_fields_for(suite + " B");
    f3.set_long(PrimaryFields::kNumberIntId, 20);
    auto created = airtable.primary().dict().create_many(std::vector<Fields>{f1, f2, f3});
    const auto ids = ids_of(created);
    const auto scope = scope_to(ids);
    try {
        const auto numbers = [](const std::vector<DictTable::Record>& records) {
            std::vector<int64_t> out;
            for (const auto& record : records) {
                if (const auto value = record.fields.get_long(PrimaryFields::kNumberIntId)) {
                    out.push_back(*value);
                }
            }
            return out;
        };

        auto asc = airtable.primary().dict().get_many(AirtableQuery{
            .formula = scope, .sorts = {{.field = std::string(PrimaryFields::kNumberIntId)}}});
        REQUIRE(numbers(asc) == std::vector<int64_t>{10, 20, 30});

        auto desc = airtable.primary().dict().get_many(
            AirtableQuery{.formula = scope,
                          .sorts = {{.field = std::string(PrimaryFields::kNumberIntId),
                                     .direction = SortDirection::Desc}}});
        REQUIRE(numbers(desc) == std::vector<int64_t>{30, 20, 10});
    } catch (...) {
        try_remove(airtable, ids);
        throw;
    }
    try_remove(airtable, ids);
}

TEST_CASE("filter by formula: field selection", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "FieldSel");
    auto fields = primary_fields_for(suite);
    fields.set_string(PrimaryFields::kSingleLineTextId, "Hello");
    fields.set_long(PrimaryFields::kNumberIntId, 42);
    auto created = airtable.primary().dict().create_one(fields);
    try {
        auto results = airtable.primary().dict().get_many(
            AirtableQuery{.formula = "RECORD_ID()='" + created.id + "'",
                          .fields = {std::string(PrimaryFields::kPrimaryKeyId)}});
        REQUIRE(results.size() == 1);
        const auto& f = results[0].fields;
        REQUIRE(f.get_string(PrimaryFields::kPrimaryKeyId).has_value());
        REQUIRE(f.get_string(PrimaryFields::kSingleLineTextId) == std::nullopt);
        REQUIRE(f.get_long(PrimaryFields::kNumberIntId) == std::nullopt);
    } catch (...) {
        try_remove(airtable, {created.id});
        throw;
    }
    try_remove(airtable, {created.id});
}

TEST_CASE("filter by formula: lookup field filter", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "Lookup");
    auto sec_a = airtable.secondary().create_one(
        SecondaryModel{.name = suite + " Sec A", .value = "Groundwork BioAg"});
    auto sec_b = airtable.secondary().create_one(
        SecondaryModel{.name = suite + " Sec B", .value = "Groundwork Lab"});
    auto sec_c = airtable.secondary().create_one(
        SecondaryModel{.name = suite + " Sec C", .value = "Other Vendor"});
    const std::vector<std::string> sec_ids{*sec_a.id, *sec_b.id, *sec_c.id};

    auto prim_a = airtable.primary().create_one(PrimaryModel{
        .link_single = std::vector<std::string>{*sec_a.id}, .primary_key = suite + " A"});
    auto prim_b = airtable.primary().create_one(PrimaryModel{
        .link_single = std::vector<std::string>{*sec_b.id}, .primary_key = suite + " B"});
    auto prim_c = airtable.primary().create_one(PrimaryModel{
        .link_single = std::vector<std::string>{*sec_c.id}, .primary_key = suite + " C"});
    const std::vector<std::string> prim_ids{*prim_a.id, *prim_b.id, *prim_c.id};
    const auto scope = scope_to(prim_ids);
    const auto& f = PrimaryModel::F;
    try {
        REQUIRE(match_names(airtable, scope, f.lookup.contains("groundwork", false, true)) ==
                std::vector<std::string>{suite + " A", suite + " B"});
        REQUIRE(match_names(airtable, scope,
                            f.lookup.contains("nonexistent-substring-xyz", false, true))
                    .empty());
        REQUIRE(match_names(airtable, scope, f.lookup.starts_with("Ground", false, true)) ==
                std::vector<std::string>{suite + " A", suite + " B"});
        REQUIRE(match_names(airtable, scope, f.lookup.ends_with("BioAg", false, true)) ==
                std::vector<std::string>{suite + " A"});
        REQUIRE(match_names(airtable, scope, f.lookup.eq("Groundwork BioAg")) ==
                std::vector<std::string>{suite + " A"});
        REQUIRE(match_names(airtable, scope, f.lookup.not_contains("Groundwork", false, true)) ==
                std::vector<std::string>{suite + " C"});
    } catch (...) {
        try_remove(airtable, prim_ids);
        try_remove_secondaries(airtable, sec_ids);
        throw;
    }
    try_remove(airtable, prim_ids);
    try_remove_secondaries(airtable, sec_ids);
}

namespace {

/// Bare date strings must be read as UTC midnight (parity with the other
/// targets); a local-time parse shifts "First Date" off midnight and the
/// date-only column rejects it. try_parse_datetime reads date-only strings as
/// UTC midnight already.
DateTime utc(const std::string& text) {
    return *try_parse_datetime(text);
}

FormulasModel mk_formulas(const std::string& suite, const std::string& name,
                          const std::string& first, const std::string& second,
                          const std::string& third) {
    return FormulasModel{.first_date = utc(first),
                         .primary_key = suite + " " + name,
                         .second_date = utc(second),
                         .third_date = utc(third)};
}

std::set<std::string> formulas_match_names(Airtable& airtable, const std::string& suite,
                                           const std::string& scope, const std::string& formula) {
    std::set<std::string> out;
    const auto models =
        airtable.formulas().get_many(AirtableQuery{.formula = Formulas::and_({scope, formula})});
    const std::string prefix = suite + " ";
    for (const auto& model : models) {
        if (!model.primary_key.has_value()) {
            continue;
        }
        std::string name = *model.primary_key;
        if (const auto pos = name.find(prefix); pos != std::string::npos) {
            name.erase(pos, prefix.size());
        }
        out.insert(name);
    }
    return out;
}

} // namespace

TEST_CASE("filter by formula: date field vs field", "[filter][filter-by-formula]") {
    auto airtable = make_airtable();
    const auto suite = primary_key("Filter", "VsField");
    const auto& f = FormulasModel::F;

    auto created = airtable.formulas().create_many(std::vector<FormulasModel>{
        mk_formulas(suite, "Equal", "2024-06-15", "2024-06-15T00:00:00.000Z",
                    "2024-06-15T00:00:00.000Z"),
        mk_formulas(suite, "FirstBefore", "2024-01-15", "2024-06-15T00:00:00.000Z",
                    "2024-12-15T00:00:00.000Z"),
        mk_formulas(suite, "FirstAfter", "2024-12-15", "2024-06-15T00:00:00.000Z",
                    "2024-01-15T00:00:00.000Z"),
        mk_formulas(suite, "Between", "2024-06-15", "2024-01-15T00:00:00.000Z",
                    "2024-12-15T00:00:00.000Z"),
    });
    std::vector<std::string> ids;
    for (const auto& model : created) {
        ids.push_back(*model.id);
    }
    const auto scope = scope_to(ids);
    try {
        REQUIRE(formulas_match_names(airtable, suite, scope, f.first_date.on(f.second_date)) ==
                std::set<std::string>{"Equal"});
        REQUIRE(formulas_match_names(airtable, suite, scope, f.first_date.not_on(f.second_date)) ==
                std::set<std::string>{"FirstBefore", "FirstAfter", "Between"});
        REQUIRE(formulas_match_names(airtable, suite, scope, f.first_date.before(f.second_date)) ==
                std::set<std::string>{"FirstBefore"});
        REQUIRE(formulas_match_names(airtable, suite, scope, f.first_date.after(f.second_date)) ==
                std::set<std::string>{"FirstAfter", "Between"});
        REQUIRE(formulas_match_names(airtable, suite, scope,
                                     f.first_date.on_or_before(f.second_date)) ==
                std::set<std::string>{"Equal", "FirstBefore"});
        REQUIRE(
            formulas_match_names(airtable, suite, scope, f.first_date.on_or_after(f.second_date)) ==
            std::set<std::string>{"Equal", "FirstAfter", "Between"});
        REQUIRE(formulas_match_names(airtable, suite, scope,
                                     f.first_date.between(f.second_date, f.third_date)) ==
                std::set<std::string>{"Equal", "Between"});
        REQUIRE(formulas_match_names(airtable, suite, scope,
                                     f.first_date.between(f.second_date, f.third_date, false)) ==
                std::set<std::string>{"Between"});
        REQUIRE(formulas_match_names(airtable, suite, scope,
                                     f.first_date.between("2024-01-01", f.second_date)) ==
                std::set<std::string>{"Equal", "FirstBefore"});
    } catch (...) {
        try_remove_formulas(airtable, ids);
        throw;
    }
    try_remove_formulas(airtable, ids);
}
