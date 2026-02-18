from datetime import date, datetime, timezone

import pytest

from output import Airtable, FormulasModel


@pytest.fixture(scope="module")
def airtable() -> Airtable:
    return Airtable()


@pytest.fixture(scope="module")
def record(airtable: Airtable) -> FormulasModel:
    model = FormulasModel()
    model.primary_key = "New Primary Key"
    model.first_number = 10
    model.second_number = 20
    model.third_number = 30
    model.first_text = "Hello"
    model.second_text = "World"
    model.third_text = "!"
    model.first_date = date(2024, 1, 1)
    model.second_date = datetime(2024, 2, 1, tzinfo=timezone.utc)
    model.third_date = datetime(2024, 3, 1, tzinfo=timezone.utc)
    return airtable.formulas.create(model)


class TestRuntimeFormulas:
    def test_math_formula(self, record: FormulasModel):
        from_api = record.math_formula
        record.evaluate_formulas_at_runtime = True
        runtime = record.math_formula
        assert from_api == runtime

    def test_text_formula(self, record: FormulasModel):
        from_api = record.text_formula
        record.evaluate_formulas_at_runtime = True
        runtime = record.text_formula
        assert from_api == runtime

    def test_date_formula(self, record: FormulasModel):
        from_api = record.date_formula
        record.evaluate_formulas_at_runtime = True
        runtime = record.date_formula
        assert from_api == runtime

    def test_cleanup(self, record: FormulasModel):
        record.delete()
