"""
TC1 - Multi-page pagination: list more records than Airtable's 100-record default
page in a single query (no max_records) and assert the client's offset loop
reassembles every page.

Exercises the untested page-reassembly path in the generated client. Parity target
for the other 8 suites.
"""

import uuid

import pytest

from output import Airtable, PrimaryCreateRecordDict, PrimaryModel

# 105 > the 100-record default page size, so a no-max_records list spans two pages.
COUNT = 105


@pytest.fixture(scope="module")
def airtable():
    return Airtable()


def _suite(label: str) -> str:
    """A primary-key prefix unique to this run (distinct per scenario via `label`)."""
    return f"Python Pagination {label} {uuid.uuid4().hex}"


def _try_delete_many(airtable: Airtable, ids: list[str], suite: str) -> None:
    if ids:
        try:
            airtable.primary.dict.delete(record_ids=ids)
            return
        except Exception:
            pass  # fall through to a prefix sweep
    try:
        stray = airtable.primary.dict.get(formula=f'FIND("{suite}", {{Primary Key}})')
        if stray:
            airtable.primary.dict.delete(record_ids=[r["id"] for r in stray])
    except Exception:
        pass  # best-effort cleanup


def test_orm_get_spanning_multiple_pages_returns_every_record(airtable: Airtable):
    suite = _suite("Orm")
    models = []
    for i in range(1, COUNT + 1):
        m = PrimaryModel()
        m.primary_key = f"{suite} {i}"
        models.append(m)
    created_ids: list[str] = []
    try:
        created = airtable.primary.create(models)
        created_ids = [r.id for r in created]
        assert len(created) == COUNT

        # No max_records: the offset loop must walk both pages and return all 105.
        results = airtable.primary.get(formula=f'FIND("{suite}", {{Primary Key}})')
        assert len(results) == COUNT
        assert {r.id for r in results} == set(created_ids)
    finally:
        _try_delete_many(airtable, created_ids, suite)


def test_dict_get_spanning_multiple_pages_returns_every_record(airtable: Airtable):
    suite = _suite("Dict")
    records: list[PrimaryCreateRecordDict] = [{"fields": {"Primary Key": f"{suite} {i}"}} for i in range(1, COUNT + 1)]
    created_ids: list[str] = []
    try:
        created = airtable.primary.dict.create(records)
        created_ids = [r["id"] for r in created]
        assert len(created) == COUNT

        results = airtable.primary.dict.get(formula=f'FIND("{suite}", {{Primary Key}})')
        assert len(results) == COUNT
        assert {r["id"] for r in results} == set(created_ids)
    finally:
        _try_delete_many(airtable, created_ids, suite)
