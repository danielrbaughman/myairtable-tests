from datetime import date, datetime, timezone

import pytest

from output import Airtable, FormulasModel


@pytest.fixture(scope="module")
def airtable() -> Airtable:
    return Airtable()


def _base(label: str) -> FormulasModel:
    """A Formulas model seeded with the same dates the C# Base() helper uses."""
    model = FormulasModel()
    model.primary_key = "Python Variety " + label
    model.first_date = date(2024, 1, 1)
    model.second_date = datetime(2024, 2, 1, tzinfo=timezone.utc)
    model.third_date = datetime(2024, 3, 1, tzinfo=timezone.utc)
    return model


def _evaluate(record: FormulasModel, attr: str) -> tuple[str | None, str | None]:
    """Read the API-computed value, then flip the runtime flag and recompute.

    The API value must be captured *before* enabling runtime evaluation, because
    enabling the flag overwrites the memoized field with the transpiled result.
    """
    from_api: str | None = getattr(record, attr)
    record.evaluate_formulas_at_runtime = True
    runtime: str | None = getattr(record, attr)
    return from_api, runtime


def _try_delete(record: FormulasModel | None) -> None:
    if record is None or record.id is None:
        return
    try:
        record.delete()
    except Exception:  # noqa: BLE001 - best-effort cleanup
        pass


class TestRuntimeFormulaVariety:
    """TC4 - Runtime-formula input variety.

    The base TestRuntimeFormulas suite only evaluates the kitchen-sink formulas
    with ONE fully-populated input set, so the IF(OR(...=BLANK())) short-circuit
    and varied inputs are never exercised. Each case here creates a Formulas
    record with a specific input set, fetches the API-computed value, and asserts
    the transpiled runtime evaluation reproduces it.

    Scope is deliberately portable: First Number is a power of 10 (LOG exact),
    Second Number is a perfect square (SQRT exact). Negatives/zero, all-blank
    text, and irrational LOG/SQRT arguments are excluded - they error inside the
    formula or differ by a ULP between V8 and the platform math lib.
    """

    # First Number: power of 10 (LOG exact). Second Number: perfect square (SQRT exact).
    NUMBER_CASES = [
        ("hundreds", 100.0, 16.0, 8.0),
        ("ones", 1.0, 4.0, 2.0),
        ("tens", 10.0, 25.0, 3.0),
    ]

    TEXT_CASES = [
        ("unicode", "café", "naïve", "日本語🎉"),
        ("whitespace", "  he llo  ", "a b", "c"),
        ("punct", "a.e-i+o", "x/y", "z"),  # exercises fixed ENCODE_URL_COMPONENT
    ]

    @pytest.mark.parametrize(("label", "a", "b", "c"), NUMBER_CASES)
    def test_math_formula_matches_api_for_varied_numbers(self, airtable: Airtable, label: str, a: float, b: float, c: float) -> None:
        model = _base("Math " + label)
        model.first_number = a
        model.second_number = b
        model.third_number = c
        model.first_text = "x"
        model.second_text = "y"
        model.third_text = "z"
        created = airtable.formulas.create(model)
        try:
            from_api, runtime = _evaluate(created, "math_formula")
            assert from_api == runtime
        finally:
            _try_delete(created)

    def test_math_formula_blank_branch_when_numbers_missing(self, airtable: Airtable) -> None:
        # First/Second Number left unset (None) -> OR(BLANK, BLANK) is true ->
        # the formula returns BLANK(). This is the IF-true short-circuit the base
        # suite never reaches.
        model = _base("Blank")
        model.first_text = "x"
        model.second_text = "y"
        model.third_text = "z"
        created = airtable.formulas.create(model)
        try:
            from_api, runtime = _evaluate(created, "math_formula")
            assert not from_api, f"API expected blank, got {from_api!r}"
            assert not runtime, f"runtime expected blank, got {runtime!r}"
        finally:
            _try_delete(created)

    @pytest.mark.parametrize(("label", "a", "b", "c"), TEXT_CASES)
    def test_text_formula_matches_api_for_varied_text(self, airtable: Airtable, label: str, a: str, b: str, c: str) -> None:
        model = _base("Text " + label)
        model.first_number = 10.0
        model.second_number = 20.0
        model.third_number = 30.0
        model.first_text = a
        model.second_text = b
        model.third_text = c
        created = airtable.formulas.create(model)
        try:
            from_api, runtime = _evaluate(created, "text_formula")
            assert from_api == runtime
        finally:
            _try_delete(created)
