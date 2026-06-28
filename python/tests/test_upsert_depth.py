"""TC10 - Upsert depth.

Mirrors the C# ``TestUpsertDepth`` suite (csharp/tests/TestUpsertDepth.cs): a multi-field merge
key (a match must agree on ALL merge fields) and the multiple-match error path (a merge key
matching more than one record is rejected). Exercises the generated ``airtable.primary.upsert``
with an explicit ``fields_to_merge_on`` (Airtable's server-side performUpsert). Insert-vs-update is
inferred from the returned record id (the generated upsert returns the upserted model(s), not a
wasCreated flag). The multiple-match rejection surfaces as ``requests.exceptions.HTTPError`` (422),
the Python analogue of C#'s ``AirtableException.ApiError`` (see test_error_paths.py).
"""

import os
import time

import pytest
from requests.exceptions import HTTPError

from output import Airtable, PrimaryModel

# Field IDs for the Primary table (Python analogue of the C# PrimaryFields.*Id merge-key constants).
PRIMARY_KEY_ID = "fldol5Q4wmQJQvPRy"
SINGLE_LINE_TEXT_ID = "fld0BL2lFo9fqcKv3"

_SUITE = f"PyUpsert {int(time.time() * 1000)}-{os.getpid()}"


@pytest.fixture(scope="module")
def airtable() -> Airtable:
    return Airtable()


def _try_delete_many(airtable: Airtable, ids: list[str]) -> None:
    if not ids:
        return
    try:
        airtable.primary.delete(record_ids=ids)
    except HTTPError:
        pass


def _model(primary_key: str, single_line_text: str, long_text: str | None = None) -> PrimaryModel:
    m = PrimaryModel()
    m.primary_key = primary_key
    m.single_line_text = single_line_text
    if long_text is not None:
        m.long_text = long_text
    return m


def test_upsert_matches_on_multiple_merge_fields(airtable: Airtable) -> None:
    suite = f"{_SUITE} MultiKey"
    ids: list[str] = []
    try:
        # Seed a record identified by the (Primary Key, Single Line Text) pair.
        seed = airtable.primary.create(_model(suite, "anchor"))
        seed_id = seed.id
        ids.append(seed_id)

        merge_on = [PRIMARY_KEY_ID, SINGLE_LINE_TEXT_ID]

        # Same pair -> UPDATE the seed (matched on BOTH fields). Inferred by the returned id.
        updated = airtable.primary.upsert(_model(suite, "anchor", "updated"), fields_to_merge_on=merge_on)
        assert not isinstance(updated, list)
        assert updated.id == seed_id
        assert updated.long_text == "updated"

        # Same Primary Key but a DIFFERENT Single Line Text -> no match on the pair -> INSERT.
        inserted = airtable.primary.upsert(_model(suite, "different"), fields_to_merge_on=merge_on)
        assert not isinstance(inserted, list)
        ids.append(inserted.id)
        assert inserted.id != seed_id
    finally:
        _try_delete_many(airtable, ids)


def test_upsert_with_multiple_matches_raises(airtable: Airtable) -> None:
    suite = f"{_SUITE} MultiMatch"
    ids: list[str] = []
    try:
        # Two records share the same Single Line Text value.
        created = airtable.primary.create([_model(f"{suite} A", "dupe"), _model(f"{suite} B", "dupe")])
        assert isinstance(created, list)
        ids.extend(r.id for r in created)

        # Upsert merging only on Single Line Text="dupe" matches BOTH -> Airtable rejects it.
        with pytest.raises(HTTPError) as exc_info:
            airtable.primary.upsert(_model("ignored", "dupe", "x"), fields_to_merge_on=[SINGLE_LINE_TEXT_ID])
        response = exc_info.value.response
        assert response is not None
        assert response.status_code == 422
    finally:
        _try_delete_many(airtable, ids)
