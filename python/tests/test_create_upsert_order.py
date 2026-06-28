"""gbs0 - ORMTable.create() must return records in INPUT order.

create() re-fetches the just-created records via get(record_ids=...), which routes to
table.all(formula=ID.in_list(...)). Airtable's all() returns records in its own natural
(table/view) order, NOT the order of the requested ids. The old code returned that list verbatim,
so for 2+ records create() (and upsert()'s no-merge branch, which zips this result positionally
against its inputs) could land fields on the wrong models.

These tests are deterministic and need no network/creds: ORMTable.create is invoked on a hand-built
stand-in ``self`` whose ``get`` deliberately returns the re-fetch in a DIFFERENT order. With the fix
(re-key by id) the result must come back in input order regardless.
"""

import sys
import types
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).parent.parent))
from output.static.orm_table import ORMTable  # noqa: E402


class _Model:
    """Minimal stand-in for a generated ORM model (only .id and a marker tag are needed)."""

    def __init__(self, tag: str):
        self.id = None
        self.tag = tag


def _table(get_fn):
    class _Orm:
        @staticmethod
        def batch_save(records):
            # pyairtable batch_create assigns ids back onto the input models in input order.
            for i, r in enumerate(records):
                r.id = f"rec{i:03d}"

    return types.SimpleNamespace(
        invalidate_cache=lambda: None,
        _orm_cls=_Orm,
        get=get_fn,
    )


def test_create_reorders_refetch_to_input_order():
    inputs = [_Model("a"), _Model("b"), _Model("c"), _Model("d")]

    def get(record_ids):
        # Simulate Airtable returning a DIFFERENT order than requested (here: reversed).
        recs = [m for m in inputs if m.id in record_ids]
        return list(reversed(recs))

    result = ORMTable.create(_table(get), records=inputs)

    assert [m.tag for m in result] == ["a", "b", "c", "d"]
    assert [m.id for m in result] == ["rec000", "rec001", "rec002", "rec003"]


def test_create_raises_if_refetch_drops_a_record():
    inputs = [_Model("a"), _Model("b")]

    def get(record_ids):
        # e.g. a view filter / read-after-write lag drops one created record.
        return [m for m in inputs if m.id in record_ids][:-1]

    with pytest.raises(RuntimeError, match="re-fetch did not return"):
        ORMTable.create(_table(get), records=inputs)
