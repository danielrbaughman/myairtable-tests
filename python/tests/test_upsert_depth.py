"""TC10 - Upsert depth.

Mirrors the C# ``TestUpsertDepth`` suite (csharp/tests/TestUpsertDepth.cs). The base CRUD
suite only covers single-record, single-merge-field upsert. This adds a multi-field merge
key (a match must agree on ALL merge fields) and the multiple-match error path (a merge key
matching more than one record is rejected by Airtable).

PARITY NOTE: the C# target exposes a first-class ``Primary.UpsertAsync(model, mergeOn)``
returning ``(WasCreated, Model)``. The Python target does NOT generate an upsert wrapper on
the ORM/dict/model layers; it instead surfaces upsert through the underlying ``pyairtable``
``Table.batch_upsert`` (reached via ``airtable.primary._table``, the same raw-table escape
hatch ``test_error_paths.py`` uses via ``.dict``). ``batch_upsert`` takes ``key_fields``
(the C# ``mergeOn``) and returns an ``UpsertResultDict`` with ``createdRecords`` /
``updatedRecords`` / ``records``; "was created" is membership of the affected record id in
``createdRecords`` (the C# ``WasCreated``). With ``use_field_ids=True`` both the record
``fields`` keys and ``key_fields`` are field IDs, matching the C# test's use of
``PrimaryFields.*Id`` constants.

The multiple-match rejection is surfaced the same way every other server-side validation
failure is in this suite: ``pyairtable`` calls ``response.raise_for_status()`` and re-raises
``requests.exceptions.HTTPError`` carrying the Airtable error envelope, asserted via
``pytest.raises(HTTPError)`` plus a 422 status check (see ``test_error_paths.py``). This is
the Python analogue of C#'s ``AirtableException.ApiError``.
"""

import os
import time

import pytest
from requests.exceptions import HTTPError

from output import Airtable

# Field IDs for the Primary table (from the generated schema). These are the Python analogue
# of the C# ``PrimaryFields.*Id`` constants used as merge keys.
PRIMARY_KEY_ID = "fldol5Q4wmQJQvPRy"
SINGLE_LINE_TEXT_ID = "fld0BL2lFo9fqcKv3"
LONG_TEXT_ID = "fld8ulc6J0W29M6La"

# Distinct per-run, per-file prefix so concurrent suites don't collide on writes.
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


def test_upsert_matches_on_multiple_merge_fields(airtable: Airtable) -> None:
    suite = f"{_SUITE} MultiKey"
    ids: list[str] = []
    try:
        # Seed a record identified by the (Primary Key, Single Line Text) pair.
        seed = airtable.primary._table.create(
            {PRIMARY_KEY_ID: suite, SINGLE_LINE_TEXT_ID: "anchor"},
            use_field_ids=True,
        )
        seed_id = seed["id"]
        ids.append(seed_id)

        merge_on = [PRIMARY_KEY_ID, SINGLE_LINE_TEXT_ID]

        # Same pair -> UPDATE the seed (matched on BOTH fields).
        update = airtable.primary._table.batch_upsert(
            [{"fields": {PRIMARY_KEY_ID: suite, SINGLE_LINE_TEXT_ID: "anchor", LONG_TEXT_ID: "updated"}}],
            key_fields=merge_on,
            use_field_ids=True,
        )
        assert update["createdRecords"] == []
        assert update["updatedRecords"] == [seed_id]
        updated_record = update["records"][0]
        assert updated_record["id"] == seed_id
        assert updated_record["fields"][LONG_TEXT_ID] == "updated"

        # Same Primary Key but a DIFFERENT Single Line Text -> no match on the pair -> INSERT.
        insert = airtable.primary._table.batch_upsert(
            [{"fields": {PRIMARY_KEY_ID: suite, SINGLE_LINE_TEXT_ID: "different"}}],
            key_fields=merge_on,
            use_field_ids=True,
        )
        insert_record = insert["records"][0]
        ids.append(insert_record["id"])
        assert insert["updatedRecords"] == []
        assert insert["createdRecords"] == [insert_record["id"]]
        assert insert_record["id"] != seed_id
    finally:
        _try_delete_many(airtable, ids)


def test_upsert_with_multiple_matches_raises(airtable: Airtable) -> None:
    suite = f"{_SUITE} MultiMatch"
    ids: list[str] = []
    try:
        # Two records share the same Single Line Text value.
        created = airtable.primary._table.batch_create(
            [
                {PRIMARY_KEY_ID: f"{suite} A", SINGLE_LINE_TEXT_ID: "dupe"},
                {PRIMARY_KEY_ID: f"{suite} B", SINGLE_LINE_TEXT_ID: "dupe"},
            ],
            use_field_ids=True,
        )
        ids.extend(r["id"] for r in created)

        # Upsert merging only on Single Line Text="dupe" matches BOTH -> Airtable rejects it.
        with pytest.raises(HTTPError) as exc_info:
            airtable.primary._table.batch_upsert(
                [{"fields": {SINGLE_LINE_TEXT_ID: "dupe", LONG_TEXT_ID: "x"}}],
                key_fields=[SINGLE_LINE_TEXT_ID],
                use_field_ids=True,
            )
        response = exc_info.value.response
        assert response is not None
        assert response.status_code == 422
    finally:
        _try_delete_many(airtable, ids)
