from datetime import date, datetime

import pytest

from output import AND, NOT, OR, XOR, Airtable, PrimaryModel


@pytest.fixture(scope="module")
def airtable():
    return Airtable()


class TestFilterByView:
    ids: list[str]

    @pytest.fixture(autouse=True, scope="class")
    def setup_and_teardown(self, airtable):
        models = []
        for i in range(5):
            m = PrimaryModel()
            m.primary_key = f"Filter Test {i}"
            models.append(m)
        for i in range(5):
            m = PrimaryModel()
            m.primary_key = f"Don't Include Test {i}"
            models.append(m)
        created = airtable.primary.create(models)
        self.__class__.ids = [r.id for r in created]
        yield
        airtable.primary.delete(record_ids=self.ids)

    def test_only_returns_records_in_view(self, airtable):
        records = airtable.primary.get(view="Filter by View")
        assert len(records) == 5
        for record in records:
            assert record.primary_key.startswith("Filter Test")


class TestFilterByIdFormula:
    ids: list[str]
    records: list[PrimaryModel]

    @pytest.fixture(autouse=True, scope="class")
    def setup_and_teardown(self, airtable):
        models = []
        for i in range(3):
            m = PrimaryModel()
            m.primary_key = f"ID Formula Test {i}"
            models.append(m)
        created = airtable.primary.create(models)
        self.__class__.ids = [r.id for r in created]
        self.__class__.records = created
        yield
        airtable.primary.delete(record_ids=self.ids)

    def test_filter_by_id_equals(self, airtable):
        formula = PrimaryModel.f.id.equals(self.records[0].id)
        records = airtable.primary.get(formula=formula)
        assert len(records) == 1
        assert records[0].id == self.records[0].id

    def test_filter_by_id_in_list_multiple(self, airtable):
        formula = PrimaryModel.f.id.in_list([self.records[0].id, self.records[1].id])
        records = airtable.primary.get(formula=formula)
        assert len(records) == 2
        ids = [r.id for r in records]
        assert self.records[0].id in ids
        assert self.records[1].id in ids

    def test_filter_by_id_in_list_single(self, airtable):
        formula = PrimaryModel.f.id.in_list([self.records[0].id])
        records = airtable.primary.get(formula=formula)
        assert len(records) == 1
        assert records[0].id == self.records[0].id

    def test_filter_by_id_in_list_empty(self, airtable):
        formula = PrimaryModel.f.id.in_list([])
        records = airtable.primary.get(formula=formula)
        assert len(records) == 0


class TestFilterByTextFieldFormula:
    ids: list[str]

    @pytest.fixture(autouse=True, scope="class")
    def setup_and_teardown(self, airtable):
        models = []
        for name in ["TextField Alpha One", "TextField Alpha Two", "TextField Beta One"]:
            m = PrimaryModel()
            m.primary_key = name
            models.append(m)
        created = airtable.primary.create(models)
        self.__class__.ids = [r.id for r in created]
        yield
        airtable.primary.delete(record_ids=self.ids)

    def test_equals(self, airtable):
        formula = PrimaryModel.f.primary_key.equals("TextField Alpha One")
        records = airtable.primary.get(formula=formula)
        assert len(records) == 1
        assert records[0].primary_key == "TextField Alpha One"

    def test_not_equals(self, airtable):
        formula = PrimaryModel.f.primary_key.not_starts_with("TextField Alpha One")
        # Use contains to scope to our test records
        scoped = AND(PrimaryModel.f.primary_key.contains("TextField"), formula)
        records = airtable.primary.get(formula=scoped)
        assert len(records) >= 2
        for record in records:
            assert record.primary_key != "TextField Alpha One"

    def test_contains(self, airtable):
        formula = PrimaryModel.f.primary_key.contains("Alpha")
        records = airtable.primary.get(formula=formula)
        assert len(records) == 2
        for record in records:
            assert "Alpha" in record.primary_key

    def test_contains_any(self, airtable):
        formula = PrimaryModel.f.primary_key.contains_any(["Alpha", "Beta"])
        records = airtable.primary.get(formula=formula)
        assert len(records) == 3

    def test_contains_all(self, airtable):
        formula = PrimaryModel.f.primary_key.contains_all(["Alpha", "One"])
        records = airtable.primary.get(formula=formula)
        assert len(records) == 1
        assert records[0].primary_key == "TextField Alpha One"

    def test_not_contains(self, airtable):
        formula = AND(
            PrimaryModel.f.primary_key.contains("TextField"),
            PrimaryModel.f.primary_key.not_contains("Alpha"),
        )
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert "Alpha" not in record.primary_key

    def test_starts_with(self, airtable):
        formula = PrimaryModel.f.primary_key.starts_with("TextField Alpha")
        records = airtable.primary.get(formula=formula)
        assert len(records) == 2
        for record in records:
            assert record.primary_key.startswith("TextField Alpha")

    def test_not_starts_with(self, airtable):
        formula = AND(
            PrimaryModel.f.primary_key.contains("TextField"),
            PrimaryModel.f.primary_key.not_starts_with("TextField Alpha"),
        )
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert not record.primary_key.startswith("TextField Alpha")


class TestFilterByNumberFieldFormula:
    ids: list[str]

    @pytest.fixture(autouse=True, scope="class")
    def setup_and_teardown(self, airtable):
        models = []
        for name, num in [("NumField Test A", 10), ("NumField Test B", 20), ("NumField Test C", 30)]:
            m = PrimaryModel()
            m.primary_key = name
            m.number_int = num
            models.append(m)
        created = airtable.primary.create(models)
        self.__class__.ids = [r.id for r in created]
        yield
        airtable.primary.delete(record_ids=self.ids)

    def test_equals(self, airtable):
        formula = PrimaryModel.f.number_int.eq(20)
        records = airtable.primary.get(formula=formula)
        assert len(records) == 1
        assert records[0].number_int == 20

    def test_not_equals(self, airtable):
        formula = PrimaryModel.f.number_int.ne(20)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 2
        for record in records:
            assert record.number_int != 20

    def test_greater_than(self, airtable):
        formula = PrimaryModel.f.number_int.gt(10)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 2
        for record in records:
            assert record.number_int > 10

    def test_less_than(self, airtable):
        formula = PrimaryModel.f.number_int.lt(30)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 2
        with_value = [r for r in records if r.number_int is not None]
        for record in with_value:
            assert record.number_int < 30

    def test_greater_than_or_equals(self, airtable):
        formula = PrimaryModel.f.number_int.gte(20)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 2
        for record in records:
            assert record.number_int >= 20

    def test_less_than_or_equals(self, airtable):
        formula = PrimaryModel.f.number_int.lte(20)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 2
        with_value = [r for r in records if r.number_int is not None]
        for record in with_value:
            assert record.number_int <= 20

    def test_between_inclusive(self, airtable):
        formula = PrimaryModel.f.number_int.between(10, 30, True)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 3
        for record in records:
            assert 10 <= record.number_int <= 30

    def test_between_exclusive(self, airtable):
        formula = PrimaryModel.f.number_int.between(10, 30, False)
        records = airtable.primary.get(formula=formula)
        assert len(records) == 1
        assert records[0].number_int == 20


class TestFilterByBooleanFieldFormula:
    ids: list[str]

    @pytest.fixture(autouse=True, scope="class")
    def setup_and_teardown(self, airtable):
        models = []
        for name, val in [("BoolField Test A", True), ("BoolField Test B", False)]:
            m = PrimaryModel()
            m.primary_key = name
            m.checkbox = val
            models.append(m)
        created = airtable.primary.create(models)
        self.__class__.ids = [r.id for r in created]
        yield
        airtable.primary.delete(record_ids=self.ids)

    def test_equals_true(self, airtable):
        formula = PrimaryModel.f.checkbox.eq(True)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.checkbox is True

    def test_equals_false(self, airtable):
        formula = PrimaryModel.f.checkbox.eq(False)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.checkbox is False or record.checkbox is None

    def test_true(self, airtable):
        formula = PrimaryModel.f.checkbox.true()
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.checkbox is True

    def test_false(self, airtable):
        formula = PrimaryModel.f.checkbox.false()
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.checkbox is False or record.checkbox is None


class TestFilterByAttachmentsFieldFormula:
    ids: list[str]

    @pytest.fixture(autouse=True, scope="class")
    def setup_and_teardown(self, airtable):
        models = []
        m1 = PrimaryModel()
        m1.primary_key = "AttachField Test A"
        m1.attachment = [  # ty:ignore[invalid-assignment]
            {"url": "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"},
        ]
        models.append(m1)
        m2 = PrimaryModel()
        m2.primary_key = "AttachField Test B"
        models.append(m2)
        created = airtable.primary.create(models)
        self.__class__.ids = [r.id for r in created]
        yield
        airtable.primary.delete(record_ids=self.ids)

    def test_not_empty(self, airtable):
        formula = PrimaryModel.f.attachment.not_empty()
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.attachment
            assert len(record.attachment) > 0

    def test_empty(self, airtable):
        formula = PrimaryModel.f.attachment.empty()
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.attachment is None or len(record.attachment) == 0


class TestFilterByDateFieldFormula:
    ids: list[str]

    @pytest.fixture(autouse=True, scope="class")
    def setup_and_teardown(self, airtable):
        models = []
        for name, date_val in [("DateField Test A", date(2024, 1, 15)), ("DateField Test B", date(2024, 6, 15)), ("DateField Test C", None)]:
            m = PrimaryModel()
            m.primary_key = name
            if date_val:
                m.date = date_val
            models.append(m)
        created = airtable.primary.create(models)
        self.__class__.ids = [r.id for r in created]
        yield
        airtable.primary.delete(record_ids=self.ids)

    def test_on(self, airtable):
        formula = PrimaryModel.f.date.on("2024-01-15")
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.date == date(2024, 1, 15)

    def test_not_on(self, airtable):
        formula = PrimaryModel.f.date.not_on("2024-01-15")
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.date != date(2024, 1, 15)

    def test_on_or_after(self, airtable):
        formula = PrimaryModel.f.date.on_or_after("2024-06-15")
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1

    def test_on_or_before(self, airtable):
        formula = PrimaryModel.f.date.on_or_before("2024-01-15")
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1

    def test_after(self, airtable):
        formula = PrimaryModel.f.date.after("2024-01-15")
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        dates = [r.date for r in records]
        assert date(2024, 6, 15) in dates

    def test_before(self, airtable):
        formula = PrimaryModel.f.date.before("2024-06-15")
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        with_date = [r for r in records if r.date is not None]
        dates = [r.date for r in with_date]
        assert date(2024, 1, 15) in dates

    def test_between_inclusive(self, airtable):
        formula = PrimaryModel.f.date.between("2024-01-15", "2024-06-15", True)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 2
        dates = [r.date for r in records]
        assert date(2024, 1, 15) in dates
        assert date(2024, 6, 15) in dates

    def test_between_exclusive(self, airtable):
        formula = PrimaryModel.f.date.between("2024-01-01", "2024-12-31", False)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 2
        dates = [r.date for r in records]
        assert date(2024, 1, 15) in dates
        assert date(2024, 6, 15) in dates

    def test_not_empty(self, airtable):
        formula = PrimaryModel.f.date.not_empty()
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.date is not None

    def test_empty(self, airtable):
        formula = PrimaryModel.f.date.empty()
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.date is None


class TestFilterByDateFieldChainedFormula:
    ids: list[str]

    @pytest.fixture(autouse=True, scope="class")
    def setup_and_teardown(self, airtable):
        models = []
        for name, date_val in [("DateChain Test A", date(2024, 1, 15)), ("DateChain Test B", date(2024, 6, 15))]:
            m = PrimaryModel()
            m.primary_key = name
            m.date = date_val
            models.append(m)
        created = airtable.primary.create(models)
        self.__class__.ids = [r.id for r in created]
        yield
        airtable.primary.delete(record_ids=self.ids)

    def test_before_days_ago(self, airtable):
        formula = PrimaryModel.f.date.before().days_ago(1)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.date is not None

    def test_after_years_ago(self, airtable):
        formula = PrimaryModel.f.date.after().years_ago(100)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.date is not None

    def test_on_days_ago(self, airtable):
        now = datetime.now()
        target = datetime(2024, 1, 15)
        diff_days = (now - target).days
        formula = PrimaryModel.f.date.on().days_ago(diff_days)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        dates = [r.date for r in records]
        assert date(2024, 1, 15) in dates

    def test_not_on_days_ago(self, airtable):
        now = datetime.now()
        target = datetime(2024, 1, 15)
        diff_days = (now - target).days
        formula = PrimaryModel.f.date.not_on().days_ago(diff_days)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        with_date = [r for r in records if r.date is not None]
        for record in with_date:
            assert record.date != date(2024, 1, 15)

    def test_on_or_after_days_ago(self, airtable):
        formula = PrimaryModel.f.date.on_or_after().days_ago(1)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.date is not None

    def test_on_or_before_years_ago(self, airtable):
        formula = PrimaryModel.f.date.on_or_before().years_ago(5)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.date is not None

    def test_before_weeks_ago(self, airtable):
        formula = PrimaryModel.f.date.before().weeks_ago(1)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.date is not None

    def test_before_months_ago(self, airtable):
        formula = PrimaryModel.f.date.before().months_ago(1)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.date is not None

    def test_before_hours_ago(self, airtable):
        formula = PrimaryModel.f.date.before().hours_ago(1)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.date is not None

    def test_before_minutes_ago(self, airtable):
        formula = PrimaryModel.f.date.before().minutes_ago(1)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.date is not None

    def test_before_seconds_ago(self, airtable):
        formula = PrimaryModel.f.date.before().seconds_ago(1)
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 1
        for record in records:
            assert record.date is not None


class TestFilterByComplexFormula:
    ids: list[str]

    @pytest.fixture(autouse=True, scope="class")
    def setup_and_teardown(self, airtable):
        models = []
        for name, num, check in [
            ("Complex Test A", 10, True),
            ("Complex Test B", 20, False),
            ("Complex Test C", 30, True),
        ]:
            m = PrimaryModel()
            m.primary_key = name
            m.number_int = num
            m.checkbox = check
            models.append(m)
        created = airtable.primary.create(models)
        self.__class__.ids = [r.id for r in created]
        yield
        airtable.primary.delete(record_ids=self.ids)

    def test_and_number_gt_checkbox_true(self, airtable):
        formula = AND(PrimaryModel.f.number_int.gt(10), PrimaryModel.f.checkbox.eq(True))
        records = airtable.primary.get(formula=formula)
        assert len(records) == 1
        assert records[0].number_int == 30
        assert records[0].checkbox is True

    def test_and_contains_number_gte(self, airtable):
        formula = AND(
            PrimaryModel.f.primary_key.contains("Complex"),
            PrimaryModel.f.number_int.gte(20),
        )
        records = airtable.primary.get(formula=formula)
        assert len(records) == 2
        names = [r.primary_key for r in records]
        assert "Complex Test B" in names
        assert "Complex Test C" in names

    def test_or_number_eq(self, airtable):
        formula = AND(
            PrimaryModel.f.primary_key.contains("Complex Test"),
            OR(PrimaryModel.f.number_int.eq(10), PrimaryModel.f.number_int.eq(30)),
        )
        records = airtable.primary.get(formula=formula)
        assert len(records) == 2
        values = [r.number_int for r in records]
        assert 10 in values
        assert 30 in values

    def test_or_checkbox_or_number(self, airtable):
        formula = AND(
            PrimaryModel.f.primary_key.contains("Complex Test"),
            OR(PrimaryModel.f.checkbox.eq(True), PrimaryModel.f.number_int.eq(20)),
        )
        records = airtable.primary.get(formula=formula)
        assert len(records) == 3
        names = [r.primary_key for r in records]
        assert "Complex Test A" in names
        assert "Complex Test B" in names
        assert "Complex Test C" in names

    def test_xor_checkbox_number(self, airtable):
        formula = AND(
            PrimaryModel.f.primary_key.contains("Complex Test"),
            XOR(PrimaryModel.f.checkbox.eq(True), PrimaryModel.f.number_int.gte(30)),
        )
        records = airtable.primary.get(formula=formula)
        assert len(records) == 1
        assert records[0].primary_key == "Complex Test A"

    def test_not_primary_key(self, airtable):
        formula = NOT(PrimaryModel.f.primary_key.equals("Complex Test A"))
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 2
        for record in records:
            assert record.primary_key != "Complex Test A"

    def test_and_or_checkbox(self, airtable):
        formula = AND(
            OR(PrimaryModel.f.number_int.eq(10), PrimaryModel.f.number_int.eq(30)),
            PrimaryModel.f.checkbox.eq(True),
        )
        records = airtable.primary.get(formula=formula)
        assert len(records) == 2
        names = [r.primary_key for r in records]
        assert "Complex Test A" in names
        assert "Complex Test C" in names

    def test_or_and_or_primary_key(self, airtable):
        formula = OR(
            AND(PrimaryModel.f.number_int.gt(10), PrimaryModel.f.checkbox.eq(True)),
            PrimaryModel.f.primary_key.equals("Complex Test A"),
        )
        records = airtable.primary.get(formula=formula)
        assert len(records) == 2
        names = [r.primary_key for r in records]
        assert "Complex Test A" in names
        assert "Complex Test C" in names

    def test_not_and(self, airtable):
        formula = NOT(AND(PrimaryModel.f.checkbox.eq(True), PrimaryModel.f.number_int.gt(20)))
        records = airtable.primary.get(formula=formula)
        assert len(records) >= 2
        for record in records:
            assert record.primary_key != "Complex Test C"

    def test_and_not_checkbox_gte(self, airtable):
        formula = AND(
            PrimaryModel.f.primary_key.contains("Complex Test"),
            NOT(PrimaryModel.f.checkbox.eq(True)),
            PrimaryModel.f.number_int.gte(20),
        )
        records = airtable.primary.get(formula=formula)
        assert len(records) == 1
        assert records[0].primary_key == "Complex Test B"
