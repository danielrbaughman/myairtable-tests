"""myairtable-hbph - the `typecast` option must reach pyairtable's write calls.

Airtable's write API accepts a per-request ``typecast`` boolean; when true Airtable coerces string
inputs to the cell's type (creating missing select options, parsing dates/numbers, etc.). ORMTable
exposes it on the public create/update/upsert methods, defaulting to False so existing behavior is
unchanged.

These tests are deterministic and need no network/creds: ORMTable.{create,update,upsert} are invoked
on a hand-built stand-in ``self`` whose ``_table`` and ``_orm_cls`` capture the kwargs the ORM passes
down. We assert ``typecast`` is forwarded when opted in and is False (the pyairtable default) by
default.

Important finding captured here: pyairtable's ``Model.batch_save`` does NOT accept ``typecast`` (3.x),
so create() routes through ``Table.batch_create`` / ``Table.create`` when typecast is requested and
through ``batch_save`` / ``Model.save`` otherwise. Both paths are exercised below.
"""

import sys
import types
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))
from output.static.orm_table import ORMTable  # noqa: E402


class _Model:
    """Minimal stand-in for a generated ORM model.

    Doubles as the ``_orm_cls`` so the list-detection in update()/create()
    (``isinstance(record[0], self._orm_cls)``) recognizes instances.
    """

    _captured: dict = {}

    def __init__(self, tag, rid=None):
        self.id = rid
        self.tag = tag
        self.saved = False

    def to_record(self):
        return {"id": self.id or "", "createdTime": "", "fields": {"tag": self.tag}}

    def save(self):
        self.saved = True

    @classmethod
    def batch_save(cls, records):
        cls._captured["batch_save_called"] = True
        for i, r in enumerate(records):
            if r.id is None:
                r.id = f"rec{i:03d}"

    @classmethod
    def from_record(cls, rec):
        return _Model(rec["fields"].get("tag"), rid=rec["id"])


def _make_self(captured):
    """Build a stand-in ORMTable instance whose _table/_orm_cls record the typecast kwarg."""

    _Model._captured = captured

    class _Table:
        def batch_create(self, records, typecast=False, use_field_ids=None):
            captured["batch_create_typecast"] = typecast
            return [{"id": f"rec{i:03d}", "createdTime": "", "fields": f} for i, f in enumerate(records)]

        def create(self, fields, typecast=False, use_field_ids=None):
            captured["create_typecast"] = typecast
            return {"id": "rec000", "createdTime": "", "fields": fields}

        def batch_update(self, records, use_field_ids=None, typecast=False):
            captured["batch_update_typecast"] = typecast
            return list(records)

        def update(self, record_id, fields, use_field_ids=None, typecast=False):
            captured["update_typecast"] = typecast
            return {"id": record_id, "createdTime": "", "fields": fields}

        def batch_upsert(self, records, key_fields, use_field_ids=None, typecast=False):
            captured["batch_upsert_typecast"] = typecast
            return {"records": [{"id": "rec000", "createdTime": "", "fields": {"tag": "a"}}]}

    def _get(record_id=None, record_ids=None):
        if record_ids is not None:
            return [_Model("t", rid=i) for i in record_ids]
        return _Model("t", rid=record_id)

    ns = types.SimpleNamespace(
        invalidate_cache=lambda: None,
        _orm_cls=_Model,
        _table=_Table(),
        _calculated_field_ids=[],
        get=_get,
    )
    # Bind the real ORMTable methods so the no-merge upsert branch (which calls self.update /
    # self.create) exercises the actual threading code rather than a stub.
    ns.update = lambda *a, **kw: ORMTable.update(ns, *a, **kw)  # ty: ignore[no-matching-overload]
    ns.create = lambda *a, **kw: ORMTable.create(ns, *a, **kw)  # ty: ignore[no-matching-overload]
    return ns


# ---- create (batch) ----


def test_create_batch_forwards_typecast_true():
    captured = {}
    inputs = [_Model("a"), _Model("b")]
    ORMTable.create(_make_self(captured), records=inputs, typecast=True)
    assert captured["batch_create_typecast"] is True
    assert "batch_save_called" not in captured  # bypassed because batch_save lacks typecast


def test_create_batch_default_uses_batch_save_not_typecast():
    captured = {}
    inputs = [_Model("a"), _Model("b")]
    ORMTable.create(_make_self(captured), records=inputs)
    assert captured.get("batch_save_called") is True
    assert "batch_create_typecast" not in captured


# ---- create (single) ----


def test_create_single_forwards_typecast_true():
    captured = {}
    rec = _Model("a")
    ORMTable.create(_make_self(captured), record=rec, typecast=True)
    assert captured["create_typecast"] is True


def test_create_single_default_uses_save():
    captured = {}
    rec = _Model("a")
    ORMTable.create(_make_self(captured), record=rec)
    assert rec.saved is True
    assert "create_typecast" not in captured


# ---- update ----


def test_update_batch_forwards_typecast():
    captured = {}
    ORMTable.update(_make_self(captured), records=[_Model("a", rid="rec1")], typecast=True)
    assert captured["batch_update_typecast"] is True


def test_update_batch_default_false():
    captured = {}
    ORMTable.update(_make_self(captured), records=[_Model("a", rid="rec1")])
    assert captured["batch_update_typecast"] is False


def test_update_single_forwards_typecast():
    captured = {}
    ORMTable.update(_make_self(captured), record=_Model("a", rid="rec1"), typecast=True)
    assert captured["update_typecast"] is True


def test_update_single_default_false():
    captured = {}
    ORMTable.update(_make_self(captured), record=_Model("a", rid="rec1"))
    assert captured["update_typecast"] is False


# ---- upsert (merge) ----


def test_upsert_merge_forwards_typecast():
    captured = {}
    ORMTable.upsert(_make_self(captured), _Model("a"), fields_to_merge_on=["fldX"], typecast=True)
    assert captured["batch_upsert_typecast"] is True


def test_upsert_merge_default_false():
    captured = {}
    ORMTable.upsert(_make_self(captured), _Model("a"), fields_to_merge_on=["fldX"])
    assert captured["batch_upsert_typecast"] is False


# ---- upsert (no-merge -> routes through update/create) ----


def test_upsert_no_merge_threads_typecast_to_update_and_create():
    captured = {}
    inputs = [_Model("has_id", rid="rec1"), _Model("no_id")]
    ORMTable.upsert(_make_self(captured), inputs, typecast=True)
    assert captured["batch_update_typecast"] is True
    assert captured["batch_create_typecast"] is True
