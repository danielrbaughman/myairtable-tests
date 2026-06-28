"""TC7 — Field-type round-trip completeness via re-fetch.

Several field types were only asserted on the create response (or only
offline-decoded), never written and read back through the live API, and
clearing/removing multi-value fields was untested. Each case here creates,
optionally updates, re-fetches, and asserts the server-side value. Parity target
for the other suites; mirrors csharp/tests/TestFieldRoundTrip.cs.
"""

import time
from datetime import datetime, timezone

import pytest

from output import Airtable, PrimaryModel

USER_ID = "usrnZ4k98m0Ipji4e"  # shared-base user, as in TestComplexProperties

URL_A = "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"
URL_B = "https://www.w3.org/Icons/w3c_home.png"


@pytest.fixture(scope="module")
def airtable():
    return Airtable()


def _try_delete(airtable: Airtable, record_id: str | None):
    if not record_id:
        return
    try:
        airtable.primary.delete(record_id=record_id)
    except Exception:
        pass


def test_date_with_time_writes_and_reads_back(airtable: Airtable):
    dt = datetime(2024, 3, 15, 14, 30, 0, tzinfo=timezone.utc)
    model = PrimaryModel()
    model.primary_key = "FieldRT DateTime"
    model.date_with_time = dt
    created = airtable.primary.create(model)
    record_id = created.id
    try:
        fetched = airtable.primary.get(record_id)
        assert fetched.date_with_time == dt
    finally:
        _try_delete(airtable, record_id)


def test_rich_text_and_percent_currency_read_back(airtable: Airtable):
    model = PrimaryModel()
    model.primary_key = "FieldRT Rich"
    model.long_text_with_rich_text = "**bold** and _italic_ text"
    model.percent_int = 0.5
    model.percent_float = 0.333
    model.currency_int = 100
    model.currency_float = 19.99
    created = airtable.primary.create(model)
    record_id = created.id
    try:
        fetched = airtable.primary.get(record_id)
        assert fetched.long_text_with_rich_text == "**bold** and _italic_ text"
        assert fetched.percent_int == 0.5
        assert fetched.percent_float == 0.333
        assert fetched.currency_int == 100
        assert fetched.currency_float == 19.99
    finally:
        _try_delete(airtable, record_id)


def test_clearing_single_and_multi_select_reads_back_empty(airtable: Airtable):
    model = PrimaryModel()
    model.primary_key = "FieldRT ClearSelect"
    model.single_select = "Choice 1"
    model.multiple_select = ["Option 1", "Option 2"]
    created = airtable.primary.create(model)
    record_id = created.id
    try:
        assert created.single_select == "Choice 1"

        created.single_select = None  # ty: ignore[invalid-assignment]
        created.multiple_select = []
        airtable.primary.update(created)

        fetched = airtable.primary.get(record_id)
        assert not fetched.single_select
        assert not fetched.multiple_select
    finally:
        _try_delete(airtable, record_id)


def test_removing_a_collaborator_reads_back_null(airtable: Airtable):
    model = PrimaryModel()
    model.primary_key = "FieldRT RemoveUser"
    model.user = {"id": USER_ID}  # ty: ignore[invalid-assignment]
    created = airtable.primary.create(model)
    record_id = created.id
    try:
        assert created.user
        assert created.user["id"] == USER_ID  # ty: ignore[invalid-key]

        created.user = None
        airtable.primary.update(created)

        fetched = airtable.primary.get(record_id)
        assert not fetched.user
    finally:
        _try_delete(airtable, record_id)


def test_attachment_replace_and_remove_read_back(airtable: Airtable):
    model = PrimaryModel()
    model.primary_key = "FieldRT Attach"
    model.attachment = [{"url": URL_A}]  # ty: ignore[invalid-assignment]
    created = airtable.primary.create(model)
    record_id = created.id
    try:
        # Replace the attachment with a different one.
        created.attachment = [{"url": URL_B}]  # ty: ignore[invalid-assignment]
        airtable.primary.update(created)
        replaced = None
        for _ in range(10):
            time.sleep(5)
            replaced = airtable.primary.get(record_id)
            if replaced.attachment:
                break
        assert replaced is not None
        assert len(replaced.attachment) == 1

        # Remove all attachments.
        replaced.attachment = []
        airtable.primary.update(replaced)
        cleared = airtable.primary.get(record_id)
        assert not cleared.attachment
    finally:
        _try_delete(airtable, record_id)
