from datetime import date, datetime, timedelta, timezone

import pytest

from output import Airtable, PrimaryModel, SecondaryModel


@pytest.fixture(scope="module")
def airtable():
    return Airtable()


class TestSimpleFieldsToRecord:
    id: str
    model: PrimaryModel

    def test_setup(self, airtable: Airtable):
        model = PrimaryModel()
        model.primary_key = "Serialize Simple Test"
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
        self.__class__.model = model

    def test_to_record_has_id(self):
        r = self.model.to_record()
        assert r["id"] == self.model.id

    def test_to_record_uses_field_ids_as_keys(self):
        r = self.model.to_record()
        # PrimaryModel has use_field_ids=True, so to_record() returns field IDs as keys
        # "Primary Key" field ID is "fldol5Q4wmQJQvPRy"
        assert r["fields"]["fldol5Q4wmQJQvPRy"] == "Serialize Simple Test"
        # "Single Line Text" field ID is "fld0BL2lFo9fqcKv3"
        assert r["fields"]["fld0BL2lFo9fqcKv3"] == "Hello World"
        # "Checkbox" field ID is "fldjQIaAZVegb1FUa"
        assert r["fields"]["fldjQIaAZVegb1FUa"] is True

    def test_to_record_dict_returns_same(self):
        r = self.model.to_record_dict()
        assert r["id"] == self.model.id
        assert r["fields"]["fldol5Q4wmQJQvPRy"] == "Serialize Simple Test"

    def test_cleanup(self, airtable: Airtable):
        self.model.delete()


class TestLinkedRecordsToRecord:
    model: PrimaryModel
    sec1: SecondaryModel
    sec2: SecondaryModel

    def test_setup(self, airtable: Airtable):
        sec1 = SecondaryModel()
        sec1.name = "Ser Link 1"
        sec1.value = "val1"
        sec1.save()
        sec2 = SecondaryModel()
        sec2.name = "Ser Link 2"
        sec2.value = "val2"
        sec2.save()
        self.__class__.sec1 = sec1
        self.__class__.sec2 = sec2

        model = PrimaryModel()
        model.primary_key = "Serialize Link Test"
        model.link_single = sec1
        model.link_multiple = [sec1, sec2]
        model.save()
        self.__class__.model = model

    def test_to_record_linked_fields(self):
        r = self.model.to_record()
        # "Link (single)" field ID is "fld7F5onkDo6mkmbN"
        assert r["fields"]["fld7F5onkDo6mkmbN"] == [self.sec1.id]
        # "Link (multiple)" field ID is "fldFyFheQWczd8oux"
        assert r["fields"]["fldFyFheQWczd8oux"] == [self.sec1.id, self.sec2.id]

    def test_cleanup(self, airtable: Airtable):
        self.model.delete()
        self.sec1.delete()
        self.sec2.delete()


class TestUnfetchedModel:
    def test_to_record_before_saving(self):
        m = PrimaryModel()
        m.primary_key = "Unsaved Model"
        r = m.to_record()
        assert r["fields"]["fldol5Q4wmQJQvPRy"] == "Unsaved Model"

    def test_to_record_dict_before_saving(self):
        m = PrimaryModel()
        m.primary_key = "Unsaved Model"
        r = m.to_record_dict()
        assert r["fields"]["fldol5Q4wmQJQvPRy"] == "Unsaved Model"
