"""TC9 — Cache thread-safety.

The TTL cache is shared mutable state on the client; the caching suite only
exercises it single-threaded. These fire concurrent reads and concurrent
mutations at one cached client and assert no corruption/exception and a
consistent result.

The generated Python Airtable has process-global/class-level shared state
(``set_airtable_config`` plus a class-level ``_tables`` cache). If concurrent
access corrupts that shared state or raises, that is a real finding.

Mirrors csharp/tests/TestCacheConcurrency.cs.
"""

import uuid
from concurrent.futures import ThreadPoolExecutor

import pytest

from output import Airtable, PrimaryModel


@pytest.fixture(scope="module")
def airtable_cached():
    return Airtable(cache_seconds=60)


def _primary_key(scenario: str) -> str:
    return f"CacheConc {scenario} {uuid.uuid4().hex[:8]}"


def _try_delete(airtable: Airtable, record_id: str | None) -> None:
    if not record_id:
        return
    try:
        airtable.primary.delete(record_id=record_id)
    except Exception:
        pass


def test_concurrent_gets_of_the_same_record_are_consistent(airtable_cached: Airtable):
    key = _primary_key("Reads")
    model = PrimaryModel()
    model.primary_key = key
    created = airtable_cached.primary.create(model)
    record_id = created.id

    def get_one() -> PrimaryModel:
        return airtable_cached.primary.get(record_id)

    try:
        # 25 concurrent gets race to populate/read the shared cache.
        with ThreadPoolExecutor(max_workers=25) as pool:
            futures = [pool.submit(get_one) for _ in range(25)]
            results = [f.result() for f in futures]
        assert all(m.primary_key == key for m in results)
        assert all(m.id == record_id for m in results)
    finally:
        _try_delete(airtable_cached, record_id)


def test_concurrent_reads_and_mutations_do_not_corrupt_the_cache(airtable_cached: Airtable):
    key = _primary_key("Mix")
    model = PrimaryModel()
    model.primary_key = key
    created = airtable_cached.primary.create(model)
    record_id = created.id

    def get_one() -> PrimaryModel:
        return airtable_cached.primary.get(record_id)

    try:
        # Interleave reads (populate cache), per-table invalidations, and
        # cascade (invalidate-all) invalidations concurrently.
        with ThreadPoolExecutor(max_workers=16) as pool:
            futures = []
            for _ in range(15):
                futures.append(pool.submit(get_one))
                futures.append(pool.submit(airtable_cached.primary.invalidate_cache))
                futures.append(pool.submit(airtable_cached.invalidate_cache))
            for f in futures:
                f.result()

        # The client is still usable and returns the correct value afterward.
        fetched = get_one()
        assert fetched.primary_key == key
    finally:
        _try_delete(airtable_cached, record_id)
