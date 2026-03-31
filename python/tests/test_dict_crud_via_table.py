import re
import time

import pytest

from output import Airtable


@pytest.fixture(scope="module")
def airtable():
    return Airtable()


def new_primary_dict(fields=None):
    return {"fields": fields or {}}


def new_secondary_dict(fields=None):
    return {"fields": fields or {}}


class TestPrimaryKeyOnly:
    id: str

    def test_create(self, airtable):
        record = new_primary_dict({"Primary Key": "New Primary Key"})
        created = airtable.primary.dict.create(record)
        self.__class__.id = created["id"]
        assert created["id"]
        assert created["fields"]["Primary Key"] == "New Primary Key"

    def test_read(self, airtable):
        read = airtable.primary.dict.get(self.id)
        assert read["id"] == self.id
        assert read["fields"]["Primary Key"] == "New Primary Key"

    def test_update(self, airtable):
        r = airtable.primary.dict.get(self.id)
        r["fields"]["Primary Key"] = "Updated Primary Key"
        updated = airtable.primary.dict.update(r)
        assert updated["id"] == self.id
        assert updated["fields"]["Primary Key"] == "Updated Primary Key"

    def test_delete(self, airtable):
        airtable.primary.dict.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.dict.get(self.id)


class TestAllSimpleProperties:
    id: str

    def test_create(self, airtable):
        record = new_primary_dict(
            {
                "Primary Key": "All Props Key",
                "Single Line Text": "Hello World",
                "Long Text": "Long text content",
                "Long Text with Rich Text": "Rich text content",
                "Email": "test@example.com",
                "URL": "https://example.com",
                "Phone Number": "555-1234",
                "Checkbox": True,
                "Number (int)": 42,
                "Number (float)": 3.14,
                "Currency (int)": 10,
                "Currency (float)": 9.99,
                "Percent (int)": 0.5,
                "Percent (float)": 0.333,
                "Duration": 3600,
                "Rating": 3,
                "Date": "2025-01-15",
                "Date (with time)": "2025-01-15T10:00:00.000Z",
                "Single Select": "Choice 1",
                "Multiple Select": ["Option 1", "Option 2"],
            }
        )
        created = airtable.primary.dict.create(record)
        self.__class__.id = created["id"]
        assert created["id"]
        f = created["fields"]
        assert f["Primary Key"] == "All Props Key"
        assert f["Single Line Text"] == "Hello World"
        assert f["Long Text"] == "Long text content"
        assert f["Long Text with Rich Text"] == "Rich text content"
        assert f["Email"] == "test@example.com"
        assert f["URL"] == "https://example.com"
        assert f["Phone Number"] == "555-1234"
        assert f["Checkbox"] is True
        assert f["Number (int)"] == 42
        assert f["Number (float)"] == 3.14
        assert f["Currency (int)"] == 10
        assert f["Currency (float)"] == 9.99
        assert f["Percent (int)"] == 0.5
        assert f["Percent (float)"] == 0.333
        assert f["Duration"] == 3600
        assert f["Rating"] == 3
        assert f["Date"] == "2025-01-15"
        assert f["Date (with time)"] == "2025-01-15T10:00:00.000Z"
        assert f["Single Select"] == "Choice 1"
        assert f["Multiple Select"] == ["Option 1", "Option 2"]

    def test_read(self, airtable):
        read = airtable.primary.dict.get(self.id)
        assert read["id"] == self.id
        f = read["fields"]
        assert f["Primary Key"] == "All Props Key"
        assert f["Single Line Text"] == "Hello World"
        assert f["Long Text"] == "Long text content"
        assert f["Long Text with Rich Text"] == "Rich text content"
        assert f["Email"] == "test@example.com"
        assert f["URL"] == "https://example.com"
        assert f["Phone Number"] == "555-1234"
        assert f["Checkbox"] is True
        assert f["Number (int)"] == 42
        assert f["Number (float)"] == 3.14
        assert f["Currency (int)"] == 10
        assert f["Currency (float)"] == 9.99
        assert f["Percent (int)"] == 0.5
        assert f["Percent (float)"] == 0.333
        assert f["Duration"] == 3600
        assert f["Rating"] == 3
        assert f["Date"] == "2025-01-15"
        assert f["Date (with time)"] == "2025-01-15T10:00:00.000Z"
        assert f["Single Select"] == "Choice 1"
        assert f["Multiple Select"] == ["Option 1", "Option 2"]

    def test_update(self, airtable):
        r = airtable.primary.dict.get(self.id)
        r["fields"]["Primary Key"] = "Updated All Props Key"
        r["fields"]["Single Line Text"] = "Updated Hello"
        r["fields"]["Long Text"] = "Updated long text"
        r["fields"]["Long Text with Rich Text"] = "Updated rich text"
        r["fields"]["Email"] = "updated@example.com"
        r["fields"]["URL"] = "https://updated.com"
        r["fields"]["Phone Number"] = "555-5678"
        r["fields"]["Checkbox"] = False
        r["fields"]["Number (int)"] = 100
        r["fields"]["Number (float)"] = 2.72
        r["fields"]["Currency (int)"] = 20
        r["fields"]["Currency (float)"] = 19.99
        r["fields"]["Percent (int)"] = 0.75
        r["fields"]["Percent (float)"] = 0.667
        r["fields"]["Duration"] = 7200
        r["fields"]["Rating"] = 5
        r["fields"]["Date"] = "2025-06-15"
        r["fields"]["Date (with time)"] = "2025-06-15T14:00:00.000Z"
        r["fields"]["Single Select"] = "Choice 2"
        r["fields"]["Multiple Select"] = ["Option 2", "Option 3"]
        updated = airtable.primary.dict.update(r)
        assert updated["id"] == self.id
        f = updated["fields"]
        assert f["Primary Key"] == "Updated All Props Key"
        assert f["Single Line Text"] == "Updated Hello"
        assert f["Long Text"] == "Updated long text"
        assert f["Long Text with Rich Text"] == "Updated rich text"
        assert f["Email"] == "updated@example.com"
        assert f["URL"] == "https://updated.com"
        assert f["Phone Number"] == "555-5678"
        assert not f.get("Checkbox")
        assert f["Number (int)"] == 100
        assert f["Number (float)"] == 2.72
        assert f["Currency (int)"] == 20
        assert f["Currency (float)"] == 19.99
        assert f["Percent (int)"] == 0.75
        assert f["Percent (float)"] == 0.667
        assert f["Duration"] == 7200
        assert f["Rating"] == 5
        assert f["Date"] == "2025-06-15"
        assert f["Date (with time)"] == "2025-06-15T14:00:00.000Z"
        assert f["Single Select"] == "Choice 2"
        assert f["Multiple Select"] == ["Option 2", "Option 3"]

    def test_delete(self, airtable):
        airtable.primary.dict.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.dict.get(self.id)


class TestComplexPropertiesLinkedRecords:
    sec1_id: str
    sec2_id: str
    id: str

    def test_setup_secondary(self, airtable):
        sec1 = airtable.secondary.dict.create(new_secondary_dict({"Name": "Link Target 1", "Value": "val1"}))
        sec2 = airtable.secondary.dict.create(new_secondary_dict({"Name": "Link Target 2", "Value": "val2"}))
        self.__class__.sec1_id = sec1["id"]
        self.__class__.sec2_id = sec2["id"]

    def test_create(self, airtable):
        record = new_primary_dict(
            {
                "Primary Key": "Link Test",
                "Link (single)": [self.sec1_id],
                "Link (multiple)": [self.sec1_id, self.sec2_id],
            }
        )
        created = airtable.primary.dict.create(record)
        self.__class__.id = created["id"]
        assert created["id"]
        assert created["fields"]["Link (single)"] == [self.sec1_id]
        assert created["fields"]["Link (multiple)"] == [self.sec1_id, self.sec2_id]

    def test_read(self, airtable):
        read = airtable.primary.dict.get(self.id)
        assert read["fields"]["Link (single)"] == [self.sec1_id]
        assert read["fields"]["Link (multiple)"] == [self.sec1_id, self.sec2_id]

    def test_update(self, airtable):
        r = airtable.primary.dict.get(self.id)
        r["fields"]["Link (single)"] = [self.sec2_id]
        r["fields"]["Link (multiple)"] = [self.sec1_id]
        updated = airtable.primary.dict.update(r)
        assert updated["fields"]["Link (single)"] == [self.sec2_id]
        assert updated["fields"]["Link (multiple)"] == [self.sec1_id]

    def test_delete(self, airtable):
        airtable.primary.dict.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.dict.get(self.id)

    def test_cleanup(self, airtable):
        airtable.secondary.dict.delete(record_id=self.sec1_id)
        airtable.secondary.dict.delete(record_id=self.sec2_id)


class TestComplexPropertiesAttachments:
    id: str

    def test_create(self, airtable):
        record = new_primary_dict(
            {
                "Primary Key": "Attachment Test",
                "Attachment": [
                    {"url": "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"},
                ],
            }
        )
        created = airtable.primary.dict.create(record)
        self.__class__.id = created["id"]
        assert created["id"]
        assert len(created["fields"]["Attachment"]) == 1
        assert created["fields"]["Attachment"][0]["url"]

    def test_read(self, airtable):
        read = None
        for _ in range(10):
            time.sleep(5)
            read = airtable.primary.dict.get(self.id)
            if read["fields"].get("Attachment"):
                break
        assert read is not None
        assert len(read["fields"]["Attachment"]) == 1
        assert read["fields"]["Attachment"][0]["url"]

    def test_delete(self, airtable):
        airtable.primary.dict.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.dict.get(self.id)


class TestComplexPropertiesUser:
    id: str

    def test_create(self, airtable):
        record = new_primary_dict(
            {
                "Primary Key": "User Test",
                "User": {
                    "id": "usrnZ4k98m0Ipji4e",
                    "email": "9vymqckyxq@privaterelay.appleid.com",
                    "name": "Daniel Baughman",
                },
                "User (allow multiple)": [
                    {"id": "usrnZ4k98m0Ipji4e", "email": "9vymqckyxq@privaterelay.appleid.com", "name": "Daniel Baughman"},
                ],
            }
        )
        created = airtable.primary.dict.create(record)
        self.__class__.id = created["id"]
        assert created["id"]
        assert created["fields"]["User"]
        assert created["fields"]["User"]["id"] == "usrnZ4k98m0Ipji4e"
        assert len(created["fields"]["User (allow multiple)"]) == 1
        assert created["fields"]["User (allow multiple)"][0]["id"] == "usrnZ4k98m0Ipji4e"

    def test_read(self, airtable):
        read = airtable.primary.dict.get(self.id)
        assert read["fields"]["User"]
        assert read["fields"]["User"]["id"] == "usrnZ4k98m0Ipji4e"
        assert len(read["fields"]["User (allow multiple)"]) == 1
        assert read["fields"]["User (allow multiple)"][0]["id"] == "usrnZ4k98m0Ipji4e"

    def test_delete(self, airtable):
        airtable.primary.dict.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.dict.get(self.id)


class TestComplexPropertiesComputedFields:
    id: str

    def test_create(self, airtable):
        record = new_primary_dict(
            {
                "Primary Key": "Computed Test",
                "Number (int)": 10,
                "Number (float)": 5,
            }
        )
        created = airtable.primary.dict.create(record)
        self.__class__.id = created["id"]
        assert created["id"]
        assert isinstance(created["fields"]["Auto Number"], (int, float))
        assert created["fields"]["Created Time"]
        assert created["fields"]["Formula (ID)"]
        assert created["fields"]["Formula (Simple)"] == 15

    def test_read(self, airtable):
        read = airtable.primary.dict.get(self.id)
        assert isinstance(read["fields"]["Auto Number"], (int, float))
        assert read["fields"]["Created Time"]
        assert read["fields"]["Formula (ID)"] == self.id
        assert read["fields"]["Formula (Simple)"] == 15

    def test_delete(self, airtable):
        airtable.primary.dict.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.dict.get(self.id)


class TestBatchOperations:
    ids: list[str]

    def test_create(self, airtable):
        count = 111
        records = [new_primary_dict({"Primary Key": f"Batch Record {i + 1}"}) for i in range(count)]
        created = airtable.primary.dict.create(records)
        self.__class__.ids = [r["id"] for r in created]
        assert len(created) == count
        for r in created:
            assert r["id"]
        for i in range(count):
            assert created[i]["fields"]["Primary Key"] == f"Batch Record {i + 1}"

    def test_read(self, airtable):
        read = airtable.primary.dict.get(self.ids)
        assert len(read) == 111
        for r in read:
            assert re.match(r"^Batch Record \d+$", r["fields"]["Primary Key"])

    def test_update(self, airtable):
        fetched = airtable.primary.dict.get(self.ids)
        for i, r in enumerate(fetched):
            r["fields"]["Primary Key"] = f"Updated Batch Record {i + 1}"
        updated = airtable.primary.dict.update(fetched)
        assert len(updated) == 111
        for i in range(111):
            assert updated[i]["fields"]["Primary Key"] == f"Updated Batch Record {i + 1}"

    def test_delete(self, airtable):
        airtable.primary.dict.delete(record_ids=self.ids)
        remaining = airtable.primary.dict.get(self.ids)
        assert len(remaining) == 0


class TestInvalidRecordId:
    def test_empty_string_id(self, airtable):
        with pytest.raises(Exception):
            airtable.primary.dict.get("")

    def test_invalid_id(self, airtable):
        with pytest.raises(Exception):
            airtable.primary.dict.get("rec_INVALID_ID")


class TestFieldSelection:
    id: str

    def test_create(self, airtable):
        record = new_primary_dict({"Primary Key": "Field Select", "Single Line Text": "Hello"})
        created = airtable.primary.dict.create(record)
        self.__class__.id = created["id"]
        assert created["id"]

    def test_read_with_fields(self, airtable):
        read = airtable.primary.dict.get(self.id, fields=["Primary Key"])
        assert read["fields"]["Primary Key"] == "Field Select"
        assert "Single Line Text" not in read["fields"]

    def test_delete(self, airtable):
        airtable.primary.dict.delete(record_id=self.id)
        with pytest.raises(Exception):
            airtable.primary.dict.get(self.id)


class TestMaxRecords:
    ids: list[str]

    def test_create(self, airtable):
        records = [new_primary_dict({"Primary Key": f"Max Records {i + 1}"}) for i in range(5)]
        created = airtable.primary.dict.create(records)
        self.__class__.ids = [r["id"] for r in created]
        assert len(created) == 5

    def test_read_with_max_records(self, airtable):
        read = airtable.primary.dict.get(self.ids, max_records=3)
        assert len(read) == 3

    def test_delete(self, airtable):
        airtable.primary.dict.delete(record_ids=self.ids)
        remaining = airtable.primary.dict.get(self.ids)
        assert len(remaining) == 0
