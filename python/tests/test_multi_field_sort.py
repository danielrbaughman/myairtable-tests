"""TC11 — Multi-field sort + sort combined with a filter.

The base filter suite only covers a single-field sort. This verifies a two-field
sort (primary key with ties broken by a secondary key) and sorting within a
filtered scope. Parity target for the other 8 suites.
"""

import pytest

from output import AND, Airtable, PrimaryModel
from output.static.table_helpers import SortOption

# Field-name sort keys (PrimaryField literals); use_field_ids defaults true on
# get(), but Airtable accepts field names in the sort param regardless.
NUMBER_INT = "Number (int)"
SINGLE_LINE_TEXT = "Single Line Text"


@pytest.fixture(scope="module")
def airtable():
    return Airtable()


def _row(number: int, text: str) -> PrimaryModel:
    m = PrimaryModel()
    m.primary_key = f"Sort {text}"
    m.number_int = number
    m.single_line_text = text
    return m


def _texts(records: list[PrimaryModel]) -> list[str]:
    return [r.single_line_text for r in records]


class TestMultiFieldSort:
    def test_two_field_sort_breaks_ties_on_secondary_key(self, airtable: Airtable):
        # number_int ties at 10 (rows "b" and "a"); secondary single_line_text orders them.
        created = airtable.primary.create([_row(10, "b"), _row(10, "a"), _row(20, "c")])
        ids = [r.id for r in created]
        try:
            sort: list[SortOption] = [
                {"field": NUMBER_INT, "direction": "asc"},
                {"field": SINGLE_LINE_TEXT, "direction": "asc"},
            ]
            results = airtable.primary.get(formula=PrimaryModel.f.id.in_list(ids), sort=sort)
            # (10,a), (10,b), (20,c) — tie on 10 broken by text asc.
            assert _texts(results) == ["a", "b", "c"]
        finally:
            airtable.primary.delete(record_ids=ids)

    def test_secondary_descending_reverses_tied_group(self, airtable: Airtable):
        created = airtable.primary.create([_row(10, "a"), _row(10, "b"), _row(20, "c")])
        ids = [r.id for r in created]
        try:
            sort: list[SortOption] = [
                {"field": NUMBER_INT, "direction": "asc"},
                {"field": SINGLE_LINE_TEXT, "direction": "desc"},
            ]
            results = airtable.primary.get(formula=PrimaryModel.f.id.in_list(ids), sort=sort)
            # number_int asc, then text DESC within the 10-tie: (10,b), (10,a), (20,c).
            assert _texts(results) == ["b", "a", "c"]
        finally:
            airtable.primary.delete(record_ids=ids)

    def test_sort_combined_with_a_filter(self, airtable: Airtable):
        created = airtable.primary.create([_row(30, "x"), _row(10, "y"), _row(20, "z"), _row(5, "low")])
        ids = [r.id for r in created]
        try:
            formula = AND(PrimaryModel.f.id.in_list(ids), PrimaryModel.f.number_int.gt(5))
            sort: list[SortOption] = [{"field": NUMBER_INT, "direction": "asc"}]
            results = airtable.primary.get(formula=formula, sort=sort)
            # Filtered to number_int > 5, sorted asc: 10(y), 20(z), 30(x). "low" filtered out.
            assert _texts(results) == ["y", "z", "x"]
        finally:
            airtable.primary.delete(record_ids=ids)
