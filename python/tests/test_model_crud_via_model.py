import time
from datetime import date, datetime, timedelta, timezone

import pytest

from output import Airtable, PrimaryModel, SecondaryModel, TertiaryModel


@pytest.fixture(scope="module")
def airtable():
    return Airtable()


class TestPrimaryKeyOnly:
    id: str

    def test_create(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "New Primary Key"
        model.save()
        self.__class__.id = model.id
        assert model.id
        assert model.primary_key == "New Primary Key"

    def test_read(self, airtable: Airtable):
        read = airtable.primary.get(self.id)
        assert read.id == self.id
        assert read.primary_key == "New Primary Key"

    def test_update(self, airtable: Airtable):
        r = airtable.primary.get(self.id)
        r.primary_key = "Updated Primary Key"
        r.save()
        assert r.id == self.id
        assert r.primary_key == "Updated Primary Key"

    def test_delete(self, airtable: Airtable):
        r = airtable.primary.get(self.id)
        r.delete()
        remaining = airtable.primary.get([self.id])
        assert len(remaining) == 0


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
        model.save()
        self.__class__.id = model.id
        assert model.id
        assert model.primary_key == "All Props Key"
        assert model.single_line_text == "Hello World"
        assert model.long_text == "Long text content"
        assert model.long_text_with_rich_text == "Rich text content"
        assert model.email == "test@example.com"
        assert model.url == "https://example.com"
        assert model.phone_number == "555-1234"
        assert model.checkbox is True
        assert model.number_int == 42
        assert model.number_float == 3.14
        assert model.currency_int == 10
        assert model.currency_float == 9.99
        assert model.percent_int == 0.5
        assert model.percent_float == 0.333
        assert model.duration == timedelta(seconds=3600)

        assert model.date == date(2025, 1, 15)
        assert model.date_with_time == datetime(2025, 1, 15, 10, 0, 0, tzinfo=timezone.utc)
        assert model.single_select == "Choice 1"
        assert model.multiple_select == ["Option 1", "Option 2"]

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
        r.save()
        assert r.id == self.id
        assert r.primary_key == "Updated All Props Key"
        assert r.single_line_text == "Updated Hello"
        assert r.long_text == "Updated long text"
        assert r.long_text_with_rich_text == "Updated rich text"
        assert r.email == "updated@example.com"
        assert r.url == "https://updated.com"
        assert r.phone_number == "555-5678"
        assert not r.checkbox
        assert r.number_int == 100
        assert r.number_float == 2.72
        assert r.currency_int == 20
        assert r.currency_float == 19.99
        assert r.percent_int == 0.75
        assert r.percent_float == 0.667
        assert r.duration == timedelta(seconds=7200)

        assert r.date == date(2025, 6, 15)
        assert r.date_with_time == datetime(2025, 6, 15, 14, 0, 0, tzinfo=timezone.utc)
        assert r.single_select == "Choice 2"
        assert r.multiple_select == ["Option 2", "Option 3"]

    def test_delete(self, airtable: Airtable):
        r = airtable.primary.get(self.id)
        r.delete()
        remaining = airtable.primary.get([self.id])
        assert len(remaining) == 0


class TestComplexPropertiesLinkedRecords:
    sec1: SecondaryModel
    sec2: SecondaryModel
    id: str

    def test_setup_secondary(self, airtable: Airtable):
        sec1 = SecondaryModel()
        sec1.name = "Link Target 1"
        sec1.value = "val1"
        sec1.save()
        sec2 = SecondaryModel()
        sec2.name = "Link Target 2"
        sec2.value = "val2"
        sec2.save()
        self.__class__.sec1 = sec1
        self.__class__.sec2 = sec2

    def test_create(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "Link Test"
        model.link_single = self.sec1
        model.link_multiple = [self.sec1, self.sec2]
        model.save()
        self.__class__.id = model.id
        assert model.id

    def test_read(self, airtable: Airtable):
        read = airtable.primary.get(self.id)
        assert read.link_single.id == self.sec1.id
        assert [m.id for m in read.link_multiple] == [self.sec1.id, self.sec2.id]

    def test_update(self, airtable: Airtable):
        r = airtable.primary.get(self.id)
        r.link_single = self.sec2
        r.link_multiple = [self.sec1]
        r.save()
        updated = airtable.primary.get(self.id)
        assert updated.link_single.id == self.sec2.id
        assert [m.id for m in updated.link_multiple] == [self.sec1.id]

    def test_delete(self, airtable: Airtable):
        r = airtable.primary.get(self.id)
        r.delete()
        remaining = airtable.primary.get([self.id])
        assert len(remaining) == 0

    def test_cleanup(self, airtable: Airtable):
        self.sec1.delete()
        self.sec2.delete()


class TestNestedLinkedRecords:
    tert1: TertiaryModel
    sec1: SecondaryModel
    primary_id: str

    def test_setup(self, airtable: Airtable):
        tert1 = TertiaryModel()
        tert1.name = "Tertiary 1"
        tert1.value = "tval1"
        tert1.save()
        self.__class__.tert1 = tert1

        sec1 = SecondaryModel()
        sec1.name = "Nested Link Target"
        sec1.value = "sval1"
        sec1.link_to_tertiary = [tert1]
        sec1.save()
        self.__class__.sec1 = sec1

        primary = PrimaryModel()
        primary.primary_key = "Nested Link Test"
        primary.link_single = sec1
        primary.save()
        self.__class__.primary_id = primary.id

    def test_traverse_primary_to_secondary(self, airtable: Airtable):
        read_primary = airtable.primary.get(self.primary_id)
        assert read_primary.link_single
        linked_secondary = airtable.secondary.get(read_primary.link_single.id)
        assert linked_secondary.id == self.sec1.id
        assert linked_secondary.name == "Nested Link Target"

    def test_traverse_secondary_to_tertiary(self, airtable: Airtable):
        read_secondary = airtable.secondary.get(self.sec1.id)
        assert read_secondary.link_to_tertiary
        linked_tertiaries = airtable.tertiary.get([m.id for m in read_secondary.link_to_tertiary])
        assert len(linked_tertiaries) == 1
        assert linked_tertiaries[0].id == self.tert1.id
        assert linked_tertiaries[0].name == "Tertiary 1"
        assert linked_tertiaries[0].value == "tval1"

    def test_cleanup(self, airtable: Airtable):
        primary = airtable.primary.get(self.primary_id)
        primary.delete()
        self.sec1.delete()
        self.tert1.delete()


class TestComplexPropertiesAttachments:
    id: str

    def test_create(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "Attachment Test"
        model.attachment = [
            {"url": "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"},
        ]
        model.save()
        self.__class__.id = model.id
        assert model.id
        assert len(model.attachment) == 1
        assert model.attachment[0]["url"]

    def test_read(self, airtable: Airtable):
        read = None
        for _ in range(10):
            time.sleep(5)
            read = airtable.primary.get(self.id)
            if read.attachment:
                break
        assert len(read.attachment) == 1
        assert read.attachment[0]["url"]

    def test_delete(self, airtable: Airtable):
        r = airtable.primary.get(self.id)
        r.delete()
        remaining = airtable.primary.get([self.id])
        assert len(remaining) == 0


class TestComplexPropertiesUser:
    id: str

    def test_create(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "User Test"
        model.user = {"id": "usrnZ4k98m0Ipji4e", "email": "9vymqckyxq@privaterelay.appleid.com", "name": "Daniel Baughman"}
        model.user_allow_multiple = [
            {"id": "usrnZ4k98m0Ipji4e", "email": "9vymqckyxq@privaterelay.appleid.com", "name": "Daniel Baughman"},
        ]
        model.save()
        self.__class__.id = model.id
        assert model.id
        assert model.user
        assert model.user["id"] == "usrnZ4k98m0Ipji4e"
        assert len(model.user_allow_multiple) == 1
        assert model.user_allow_multiple[0]["id"] == "usrnZ4k98m0Ipji4e"

    def test_read(self, airtable: Airtable):
        read = airtable.primary.get(self.id)
        assert read.user
        assert read.user["id"] == "usrnZ4k98m0Ipji4e"
        assert len(read.user_allow_multiple) == 1
        assert read.user_allow_multiple[0]["id"] == "usrnZ4k98m0Ipji4e"

    def test_delete(self, airtable: Airtable):
        r = airtable.primary.get(self.id)
        r.delete()
        remaining = airtable.primary.get([self.id])
        assert len(remaining) == 0


class TestComplexPropertiesComputedFields:
    id: str

    def test_create(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "Computed Test"
        model.number_int = 10
        model.number_float = 5
        model.save()
        self.__class__.id = model.id
        assert model.id

    def test_read(self, airtable: Airtable):
        read = airtable.primary.get(self.id)
        assert isinstance(read.auto_number, (int, float))
        assert read.created_at_time
        assert read.formula_id == self.id
        assert read.formula_simple == 15

    def test_delete(self, airtable: Airtable):
        r = airtable.primary.get(self.id)
        r.delete()
        remaining = airtable.primary.get([self.id])
        assert len(remaining) == 0
