"""TC6 — Primary "Formula (Complex)"/"Formula (Nested)" evaluated at runtime.

The base runtime suite (test_runtime_formulas.py) only covers the Formulas table; the
Primary complex formula concatenates ~35 fields through IF(field, field, "None"), a
richer transpile path never checked against the API before.

We compare the transpiled runtime-evaluated Complex formula to the API line-by-line for
the DETERMINISTIC, offline-reproducible field types (text, checkbox, single/multi select,
numbers, currency, email, url, phone). The formula also references server-computed fields
(Created/Last Modified Time + By, Auto Number, Button, Formula(ID)/(Simple)) and
link/lookup/rollup — Airtable renders those from data the offline runtime doesn't hold, so
those lines are NOT expected to match offline. See myairtable-5b0n.

This suite specifically locks in the multi-select array-join fix (myairtable-bb7f): a
multi-value field coerces to "Option 1, Option 2", not just the first element.
"""

import pytest

from output import Airtable, PrimaryModel

# Field labels whose rendering the offline runtime can reproduce exactly.
DETERMINISTIC_LABELS = [
    "Single Line Text",
    "Long Text",
    "Checkbox",
    "Multiple Select",
    "Single Select",
    "Number (int)",
    "Number (float)",
    "Currency (int)",
    "Currency (float)",
    "Email",
    "URL",
    "Phone Number",
]


@pytest.fixture(scope="module")
def airtable() -> Airtable:
    return Airtable()


def new_record() -> PrimaryModel:
    model = PrimaryModel()
    model.primary_key = "TC6 Primary Formula Runtime"
    model.single_line_text = "hello"
    model.long_text = "long text"
    model.email = "a@b.co"
    model.url = "https://x.co"
    model.phone_number = "555-1212"
    model.checkbox = True
    model.number_int = 42
    model.number_float = 3.5
    model.currency_int = 10
    model.currency_float = 9.99
    model.single_select = "Choice 1"
    model.multiple_select = ["Option 1", "Option 2"]
    return model


def line(formula: str, label: str) -> str:
    """Extract the "Label: value" line for ``label`` from a formula result."""
    for entry in formula.split("\n"):
        if entry.startswith(label + ": "):
            return entry
    return f"<missing: {label}>"


class TestPrimaryFormulaRuntime:
    def test_complex_formula_renders_deterministic_fields_like_api(self, airtable: Airtable):
        created = airtable.primary.create(new_record())
        record_id = created.id
        try:
            fetched = airtable.primary.get(record_id)
            api = fetched.formula_complex or ""
            fetched.evaluate_formulas_at_runtime = True
            runtime = fetched.formula_complex or ""

            for label in DETERMINISTIC_LABELS:
                assert line(api, label) == line(runtime, label)

            # The multi-select join is the headline fix: both sides render all options, comma-joined.
            assert line(runtime, "Multiple Select") == "Multiple Select: Option 1, Option 2"
        finally:
            airtable.primary.delete(record_id=record_id)

    def test_nested_formula_evaluates_without_throwing(self, airtable: Airtable):
        # Formula (Nested) = Formula(ID) & Formula(Simple) & Formula(Complex) — it chains three
        # COMPUTED formula fields. Offline the runtime can't reproduce computed-field values, so the
        # content isn't asserted; this confirms the transpiled nested-formula method is generated and
        # evaluates without error. See myairtable-5b0n.
        created = airtable.primary.create(new_record())
        record_id = created.id
        try:
            fetched = airtable.primary.get(record_id)
            fetched.evaluate_formulas_at_runtime = True
            runtime = fetched.formula_nested  # must not throw
            assert runtime is not None
        finally:
            airtable.primary.delete(record_id=record_id)
