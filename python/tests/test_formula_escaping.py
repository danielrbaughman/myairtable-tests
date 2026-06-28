"""TC2 - Formula-value escaping.

Filter predicates build formulas by interpolating user values into quoted string
literals; a value containing a quote, backslash, or other meta-character must be
escaped or the formula breaks (or silently mis-matches). These tests round-trip
special-character values through storage AND through the generated ``.eq()``/
``.contains()`` filter DSL against the live base, proving the escaping is correct.
Parity target mirrors csharp/tests/TestFormulaEscaping.cs.
"""

import random
import time

import pytest
from pyairtable import formulas as F  # noqa: N812

from output import AND, Airtable, PrimaryModel

# Values that break naive string interpolation into an Airtable formula literal.
# Newline is covered separately (storage only) - Airtable formula string literals
# can't carry a raw newline, so it's a storage concern, not a filter one.
SPECIAL_VALUES = [
    "O'Brien single quote",
    'say "hi" double quote',
    "back\\slash path",
    "trailing backslash\\",
    "mixed a\"b'c\\d",
    "unicode café ☕ 日本語 🎉",
]


@pytest.fixture(scope="module")
def airtable() -> Airtable:
    return Airtable()


def _primary_key(suite: str, label: str) -> str:
    """A primary-key value unique to this run (distinct per scenario via label)."""
    ts = int(time.time() * 1000)
    rand = random.randint(100000, 999999)
    return f"Python {suite} {label} {ts}-{rand}"


def _scope(suite: str) -> F.Formula:
    """FIND("<suite>", {Primary Key}) - scopes a filter to records from this run."""
    return F.FIND(suite, F.Field("Primary Key"))


def _try_delete(airtable: Airtable, record_id: str | None) -> None:
    if not record_id:
        return
    try:
        airtable.primary.delete(record_id=record_id)
    except Exception:
        # best-effort cleanup
        pass


@pytest.mark.parametrize("special", SPECIAL_VALUES)
def test_eq_filter_matches_value_with_special_chars(airtable: Airtable, special: str) -> None:
    suite = _primary_key("Escaping", "Eq")
    model = PrimaryModel()
    model.primary_key = suite
    model.single_line_text = special
    created = airtable.primary.create(model)
    record_id = created.id
    try:
        # 1. Storage round-trips the exact value.
        assert created.single_line_text == special

        # 2. The .eq() filter, scoped to this record, matches via correctly-escaped
        #    interpolation.
        formula = AND(_scope(suite), PrimaryModel.f.single_line_text.eq(special))
        results = airtable.primary.get(formula=formula)
        assert any(r.id == record_id for r in results)
        for r in results:
            assert r.single_line_text == special
    finally:
        _try_delete(airtable, record_id)


@pytest.mark.parametrize("special", SPECIAL_VALUES)
def test_contains_filter_matches_value_with_special_chars(airtable: Airtable, special: str) -> None:
    suite = _primary_key("Escaping", "Contains")
    # Embed the special token inside a longer value so contains (FIND>0) is a real
    # substring test.
    stored = "prefix " + special + " suffix"
    model = PrimaryModel()
    model.primary_key = suite
    model.single_line_text = stored
    created = airtable.primary.create(model)
    record_id = created.id
    try:
        formula = AND(_scope(suite), PrimaryModel.f.single_line_text.contains(special))
        results = airtable.primary.get(formula=formula)
        assert any(r.id == record_id for r in results)
    finally:
        _try_delete(airtable, record_id)


def test_special_characters_in_primary_key_round_trip_and_filter(airtable: Airtable) -> None:
    # The primary key itself carries special chars, and we match on it with .eq().
    suite = _primary_key("Escaping", "PK")
    pk = suite + ' O\'Brien "quote" back\\slash'
    model = PrimaryModel()
    model.primary_key = pk
    created = airtable.primary.create(model)
    record_id = created.id
    try:
        assert created.primary_key == pk
        formula = PrimaryModel.f.primary_key.eq(pk)
        results = airtable.primary.get(formula=formula)
        assert any(r.id == record_id for r in results)
        assert len(results) == 1
    finally:
        _try_delete(airtable, record_id)


def test_newline_and_tab_values_round_trip_through_storage(airtable: Airtable) -> None:
    # Newlines/tabs are a storage concern (long text), not expressible in a formula
    # literal. Verify they survive create -> fetch unchanged.
    suite = _primary_key("Escaping", "Newline")
    value = "line1\nline2\twith tab\r\nwindows"
    model = PrimaryModel()
    model.primary_key = suite
    model.long_text = value
    created = airtable.primary.create(model)
    record_id = created.id
    try:
        fetched = airtable.primary.get(record_id)
        assert fetched.long_text == value
    finally:
        _try_delete(airtable, record_id)
