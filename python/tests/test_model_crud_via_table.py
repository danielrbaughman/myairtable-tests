import re
import time
from datetime import date, datetime, timedelta, timezone

import pytest

from output import Airtable, PrimaryModel, SecondaryModel


@pytest.fixture(scope="module")
def airtable():
    return Airtable()


class TestPrimaryKeyOnly:
    id: str

    def test_create(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "New Primary Key"
        created = airtable.primary.create(model)
        self.__class__.id = created.id
        assert created.id
        assert created.primary_key == "New Primary Key"

    def test_read(self, airtable: Airtable):
        read = airtable.primary.get(self.id)
        assert read.id == self.id
        assert read.primary_key == "New Primary Key"

    def test_update(self, airtable: Airtable):
        r = airtable.primary.get(self.id)
        r.primary_key = "Updated Primary Key"
        updated = airtable.primary.update(r)
        assert updated.id == self.id
        assert updated.primary_key == "Updated Primary Key"

    def test_delete(self, airtable: Airtable):
        airtable.primary.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.get(self.id)


class TestAllSimpleProperties:
    id: str

    def test_create(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "All Props Key"
        model.single_line_text = "Hello World"
        model.long_text = "Long text content"
        model.long_text_with_rich_text = "Rich text content"
        model.email = "test@example.com"
        model.url = "https://example.com"
        model.phone_number = "555-1234"
        model.checkbox = True
        model.number_int = 42
        model.number_float = 3.14
        model.currency_int = 10
        model.currency_float = 9.99
        model.percent_int = 0.5
        model.percent_float = 0.333
        model.duration = timedelta(seconds=3600)

        model.date = date(2025, 1, 15)
        model.date_with_time = datetime(2025, 1, 15, 10, 0, 0, tzinfo=timezone.utc)
        model.single_select = "Choice 1"
        model.multiple_select = ["Option 1", "Option 2"]
        created = airtable.primary.create(model)
        self.__class__.id = created.id
        assert created.id
        assert created.primary_key == "All Props Key"
        assert created.single_line_text == "Hello World"
        assert created.long_text == "Long text content"
        assert created.long_text_with_rich_text == "Rich text content"
        assert created.email == "test@example.com"
        assert created.url == "https://example.com"
        assert created.phone_number == "555-1234"
        assert created.checkbox is True
        assert created.number_int == 42
        assert created.number_float == 3.14
        assert created.currency_int == 10
        assert created.currency_float == 9.99
        assert created.percent_int == 0.5
        assert created.percent_float == 0.333
        assert created.duration == timedelta(seconds=3600)
        assert created.date == date(2025, 1, 15)
        assert created.date_with_time == datetime(2025, 1, 15, 10, 0, 0, tzinfo=timezone.utc)
        assert created.single_select == "Choice 1"
        assert created.multiple_select == ["Option 1", "Option 2"]

    def test_read(self, airtable: Airtable):
        read = airtable.primary.get(self.id)
        assert read.id == self.id
        assert read.primary_key == "All Props Key"
        assert read.single_line_text == "Hello World"
        assert read.long_text == "Long text content"
        assert read.long_text_with_rich_text == "Rich text content"
        assert read.email == "test@example.com"
        assert read.url == "https://example.com"
        assert read.phone_number == "555-1234"
        assert read.checkbox is True
        assert read.number_int == 42
        assert read.number_float == 3.14
        assert read.currency_int == 10
        assert read.currency_float == 9.99
        assert read.percent_int == 0.5
        assert read.percent_float == 0.333
        assert read.duration == timedelta(seconds=3600)
        assert read.date == date(2025, 1, 15)
        assert read.date_with_time == datetime(2025, 1, 15, 10, 0, 0, tzinfo=timezone.utc)
        assert read.single_select == "Choice 1"
        assert read.multiple_select == ["Option 1", "Option 2"]

    def test_update(self, airtable: Airtable):
        r = airtable.primary.get(self.id)
        r.primary_key = "Updated All Props Key"
        r.single_line_text = "Updated Hello"
        r.long_text = "Updated long text"
        r.long_text_with_rich_text = "Updated rich text"
        r.email = "updated@example.com"
        r.url = "https://updated.com"
        r.phone_number = "555-5678"
        r.checkbox = False
        r.number_int = 100
        r.number_float = 2.72
        r.currency_int = 20
        r.currency_float = 19.99
        r.percent_int = 0.75
        r.percent_float = 0.667
        r.duration = timedelta(seconds=7200)
        r.date = date(2025, 6, 15)
        r.date_with_time = datetime(2025, 6, 15, 14, 0, 0, tzinfo=timezone.utc)
        r.single_select = "Choice 2"
        r.multiple_select = ["Option 2", "Option 3"]
        updated = airtable.primary.update(r)
        assert updated.id == self.id
        assert updated.primary_key == "Updated All Props Key"
        assert updated.single_line_text == "Updated Hello"
        assert updated.long_text == "Updated long text"
        assert updated.long_text_with_rich_text == "Updated rich text"
        assert updated.email == "updated@example.com"
        assert updated.url == "https://updated.com"
        assert updated.phone_number == "555-5678"
        assert not updated.checkbox
        assert updated.number_int == 100
        assert updated.number_float == 2.72
        assert updated.currency_int == 20
        assert updated.currency_float == 19.99
        assert updated.percent_int == 0.75
        assert updated.percent_float == 0.667
        assert updated.duration == timedelta(seconds=7200)
        assert updated.date == date(2025, 6, 15)
        assert updated.date_with_time == datetime(2025, 6, 15, 14, 0, 0, tzinfo=timezone.utc)
        assert updated.single_select == "Choice 2"
        assert updated.multiple_select == ["Option 2", "Option 3"]

    def test_delete(self, airtable: Airtable):
        airtable.primary.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.get(self.id)


class TestComplexPropertiesLinkedRecords:
    sec1: SecondaryModel
    sec2: SecondaryModel
    id: str

    def test_setup_secondary(self, airtable: Airtable):
        sec1 = SecondaryModel()
        sec1.name = "Link Target 1"
        sec1.value = "val1"
        sec1 = airtable.secondary.create(sec1)
        sec2 = SecondaryModel()
        sec2.name = "Link Target 2"
        sec2.value = "val2"
        sec2 = airtable.secondary.create(sec2)
        self.__class__.sec1 = sec1
        self.__class__.sec2 = sec2

    def test_create(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "Link Test"
        model.link_single = self.sec1
        model.link_multiple = [self.sec1, self.sec2]
        created = airtable.primary.create(model)
        self.__class__.id = created.id
        assert created.id
        assert created.link_single.id == self.sec1.id
        assert [m.id for m in created.link_multiple] == [self.sec1.id, self.sec2.id]

    def test_read(self, airtable: Airtable):
        read = airtable.primary.get(self.id)
        assert read.link_single.id == self.sec1.id
        assert [m.id for m in read.link_multiple] == [self.sec1.id, self.sec2.id]

    def test_update(self, airtable: Airtable):
        r = airtable.primary.get(self.id)
        r.link_single = self.sec2
        r.link_multiple = [self.sec1]
        updated = airtable.primary.update(r)
        assert updated.link_single.id == self.sec2.id
        assert [m.id for m in updated.link_multiple] == [self.sec1.id]

    def test_delete(self, airtable: Airtable):
        airtable.primary.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.get(self.id)

    def test_cleanup(self, airtable: Airtable):
        airtable.secondary.delete(record_id=self.sec1.id)
        airtable.secondary.delete(record_id=self.sec2.id)


class TestComplexPropertiesAttachments:
    id: str

    def test_create(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "Attachment Test"
        model.attachment = [  # ty: ignore[invalid-assignment]
            {"url": "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"},
        ]
        created = airtable.primary.create(model)
        self.__class__.id = created.id
        assert created.id
        assert len(created.attachment) == 1
        assert created.attachment[0]["url"]  # ty: ignore[invalid-key]

    def test_read(self, airtable: Airtable):
        read = None
        for _ in range(10):
            time.sleep(5)
            read = airtable.primary.get(self.id)
            if read.attachment:
                break
        assert read is not None
        assert len(read.attachment) == 1
        assert read.attachment[0]["url"]  # ty: ignore[invalid-key]

    def test_delete(self, airtable: Airtable):
        airtable.primary.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.get(self.id)


class TestComplexPropertiesUser:
    id: str

    def test_create(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "User Test"
        model.user = {"id": "usrnZ4k98m0Ipji4e", "email": "9vymqckyxq@privaterelay.appleid.com", "name": "Daniel Baughman"}  # ty: ignore[invalid-assignment]
        model.user_allow_multiple = [  # ty: ignore[invalid-assignment]
            {"id": "usrnZ4k98m0Ipji4e", "email": "9vymqckyxq@privaterelay.appleid.com", "name": "Daniel Baughman"},
        ]
        created = airtable.primary.create(model)
        self.__class__.id = created.id
        assert created.id
        assert created.user
        assert created.user["id"] == "usrnZ4k98m0Ipji4e"  # ty: ignore[invalid-key]
        assert len(created.user_allow_multiple) == 1
        assert created.user_allow_multiple[0]["id"] == "usrnZ4k98m0Ipji4e"  # ty: ignore[invalid-key]

    def test_read(self, airtable: Airtable):
        read = airtable.primary.get(self.id)
        assert read.user
        assert read.user["id"] == "usrnZ4k98m0Ipji4e"  # ty: ignore[invalid-key]
        assert len(read.user_allow_multiple) == 1
        assert read.user_allow_multiple[0]["id"] == "usrnZ4k98m0Ipji4e"  # ty: ignore[invalid-key]

    def test_delete(self, airtable: Airtable):
        airtable.primary.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.get(self.id)


class TestComplexPropertiesComputedFields:
    id: str

    def test_create(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "Computed Test"
        model.number_int = 10
        model.number_float = 5
        created = airtable.primary.create(model)
        self.__class__.id = created.id
        assert created.id
        assert isinstance(created.auto_number, (int, float))
        assert created.created_at_time
        assert created.formula_id
        assert created.formula_simple == 15

    def test_read(self, airtable: Airtable):
        read = airtable.primary.get(self.id)
        assert isinstance(read.auto_number, (int, float))
        assert read.created_at_time
        assert read.formula_id == self.id
        assert read.formula_simple == 15

    def test_delete(self, airtable: Airtable):
        airtable.primary.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.get(self.id)


class TestBatchOperations:
    ids: list[str]

    def test_create(self, airtable: Airtable):
        count = 111
        models = []
        for i in range(count):
            m = PrimaryModel()
            m.primary_key = f"Batch Record {i + 1}"
            models.append(m)
        created = airtable.primary.create(models)
        self.__class__.ids = [r.id for r in created]
        assert len(created) == count
        for r in created:
            assert r.id
        created_keys = {r.primary_key for r in created}
        expected_keys = {f"Batch Record {i + 1}" for i in range(count)}
        assert created_keys == expected_keys

    def test_read(self, airtable: Airtable):
        read = airtable.primary.get(self.ids)
        assert len(read) == 111
        for r in read:
            assert re.match(r"^Batch Record \d+$", r.primary_key)

    def test_update(self, airtable: Airtable):
        fetched = airtable.primary.get(self.ids)
        for i, r in enumerate(fetched):
            r.primary_key = f"Updated Batch Record {i + 1}"
        updated = airtable.primary.update(fetched)
        assert len(updated) == 111
        updated_keys = {r.primary_key for r in updated}
        expected_keys = {f"Updated Batch Record {i + 1}" for i in range(111)}
        assert updated_keys == expected_keys

    def test_delete(self, airtable: Airtable):
        airtable.primary.delete(record_ids=self.ids)
        remaining = airtable.primary.get(self.ids)
        assert len(remaining) == 0


class TestInvalidRecordId:
    def test_empty_string_id(self, airtable: Airtable):
        with pytest.raises(Exception):
            airtable.primary.get("")

    def test_invalid_id(self, airtable: Airtable):
        with pytest.raises(Exception):
            airtable.primary.get("rec_INVALID_ID")


class TestFieldSelection:
    id: str

    def test_create(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "Field Select"
        model.single_line_text = "Hello"
        created = airtable.primary.create(model)
        self.__class__.id = created.id
        assert created.id

    def test_read_with_fields(self, airtable: Airtable):
        read = airtable.primary.get(self.id, fields=["Primary Key"])
        assert read.primary_key == "Field Select"
        assert not read.single_line_text

    def test_delete(self, airtable: Airtable):
        airtable.primary.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.get(self.id)


class TestMaxRecords:
    ids: list[str]

    def test_create(self, airtable: Airtable):
        models = []
        for i in range(5):
            m = PrimaryModel()
            m.primary_key = f"Max Records {i + 1}"
            models.append(m)
        created = airtable.primary.create(models)
        self.__class__.ids = [r.id for r in created]
        assert len(created) == 5

    def test_read_with_max_records(self, airtable: Airtable):
        read = airtable.primary.get(self.ids, max_records=3)
        assert len(read) == 3

    def test_delete(self, airtable: Airtable):
        airtable.primary.delete(record_ids=self.ids)
        remaining = airtable.primary.get(self.ids)
        assert len(remaining) == 0


class TestDuplicate:
    """`duplicate()` copies a record into a brand-new one.

    Worth its own class because duplicate is the first verb that POSTs a record read back
    from Airtable, which is where the interesting failure modes live: pyairtable's
    `Model.save()` PATCHes when the model has an id (so a naive copy would update the
    source), and Airtable rejects server-returned attachment objects on create.
    """

    source_id: str
    secondary_id: str
    trash: list[str] = []

    def test_create_source(self, airtable: Airtable):
        sec = SecondaryModel()
        sec.name = "Duplicate Source Link"
        sec = airtable.secondary.create(sec)
        self.__class__.secondary_id = sec.id

        model = PrimaryModel()
        model.primary_key = "Duplicate Source"
        model.single_line_text = "copy me"
        # Stays <= 10 and != 20 on purpose, matching the TS/JS duplicate suites: the
        # filter-by-formula tests assert exact counts for `numberInt = 20` and for
        # `AND(numberInt > 10, checkbox = true)` against the one shared live base.
        model.number_int = 7
        model.rating = 3
        model.checkbox = True
        model.single_select = "Choice 1"
        model.multiple_select = ["Option 1", "Option 2"]
        model.date = date(2025, 1, 15)
        model.duration = timedelta(seconds=3600)
        model.link_single = sec
        created = airtable.primary.create(model)
        self.__class__.source_id = created.id
        assert created.id

    def test_duplicate_from_model_copies_writable_fields(self, airtable: Airtable):
        source = airtable.primary.get(self.source_id)
        copy = airtable.primary.duplicate(source)
        self.__class__.trash.append(copy.id)

        assert copy.id != source.id
        assert copy.primary_key == "Duplicate Source"  # primary field copied verbatim
        assert copy.single_line_text == "copy me"
        assert copy.number_int == 7
        assert copy.rating == 3
        assert copy.checkbox is True
        assert copy.single_select == "Choice 1"
        assert sorted(copy.multiple_select or []) == ["Option 1", "Option 2"]
        assert copy.date == source.date
        assert copy.duration == source.duration

    def test_duplicate_recomputes_rather_than_copies_computed_fields(self, airtable: Airtable):
        source = airtable.primary.get(self.source_id)
        copy = airtable.primary.duplicate(source)
        self.__class__.trash.append(copy.id)

        # The sharpest available proof that computed fields were never written: Formula (ID)
        # resolves to RECORD_ID(), so on a true copy it equals the COPY's id, not the source's.
        assert copy.formula_id == copy.id
        assert copy.auto_number != source.auto_number
        assert copy.created_at_time is not None

    def test_duplicate_copies_links_without_moving_them(self, airtable: Airtable):
        source = airtable.primary.get(self.source_id)
        copy = airtable.primary.duplicate(source)
        self.__class__.trash.append(copy.id)

        assert copy.link_single is not None
        assert copy.link_single.id == self.secondary_id
        # Airtable link fields are many-to-many underneath, so the copy is added alongside the
        # original rather than displacing it. The source must keep its link.
        source_again = airtable.primary.get(self.source_id)
        assert source_again.link_single is not None
        assert source_again.link_single.id == self.secondary_id

    def test_duplicate_by_record_id(self, airtable: Airtable):
        copy = airtable.primary.duplicate(self.source_id)
        self.__class__.trash.append(copy.id)
        assert copy.id != self.source_id
        assert copy.single_line_text == "copy me"

    def test_duplicate_batch_preserves_input_order(self, airtable: Airtable):
        other = PrimaryModel()
        other.primary_key = "Duplicate Source B"
        other = airtable.primary.create(other)
        self.__class__.trash.append(other.id)

        # get(record_ids=...) returns Airtable's table order, so this pins the re-keying.
        copies = airtable.primary.duplicate([other.id, self.source_id])
        self.__class__.trash.extend(r.id for r in copies)
        assert len(copies) == 2
        assert copies[0].primary_key == "Duplicate Source B"
        assert copies[1].primary_key == "Duplicate Source"

    def test_duplicate_leaves_the_source_untouched(self, airtable: Airtable):
        before = airtable.primary.get(self.source_id)
        copy = airtable.primary.duplicate(before)
        self.__class__.trash.append(copy.id)

        after = airtable.primary.get(self.source_id)
        assert after.id == before.id
        assert after.primary_key == before.primary_key
        assert after.auto_number == before.auto_number

    def test_duplicate_rejects_an_unsaved_model(self, airtable: Airtable):
        with pytest.raises(ValueError, match="no id"):
            airtable.primary.duplicate(PrimaryModel())

    def test_delete(self, airtable: Airtable):
        for record_id in {*self.trash, self.source_id}:
            airtable.primary.delete(record_id=record_id)
        airtable.secondary.delete(record_id=self.secondary_id)


class TestDuplicateAttachment:
    """Attachments are the one field type Airtable will not let you echo back on create."""

    source_id: str
    copy_id: str
    ATTACHMENT_URL = "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"

    @staticmethod
    def _poll_for_attachment(airtable: Airtable, record_id: str):
        """Airtable ingests attachments asynchronously; the create response has no id yet."""
        for _ in range(10):
            time.sleep(5)
            record = airtable.primary.get(record_id)
            if record.attachment and record.attachment[0].get("id"):
                return record
        return airtable.primary.get(record_id)

    def test_create_source(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "Duplicate Attachment Source"
        model.attachment = [{"url": self.ATTACHMENT_URL}]  # ty: ignore[invalid-assignment]
        created = airtable.primary.create(model)
        self.__class__.source_id = created.id
        source = self._poll_for_attachment(airtable, created.id)
        assert source.attachment and source.attachment[0].get("id")

    def test_duplicate_reingests_the_attachment_independently(self, airtable: Airtable):
        source = airtable.primary.get(self.source_id)
        copy = airtable.primary.duplicate(source)
        self.__class__.copy_id = copy.id

        copied = self._poll_for_attachment(airtable, copy.id)
        assert copied.attachment and len(copied.attachment) == 1
        # Copying by URL makes Airtable re-ingest the file and mint a fresh attachment id, so
        # the copy owns its attachment rather than aliasing the source's. Echoing the server's
        # attachment object back (or its id) would have failed with INVALID_ATTACHMENT_OBJECT.
        assert copied.attachment[0]["id"] != source.attachment[0]["id"]  # ty: ignore[invalid-key]

    def test_delete(self, airtable: Airtable):
        airtable.primary.delete(record_ids=[self.source_id, self.copy_id])
