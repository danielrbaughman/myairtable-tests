import time

import pytest

from output import Airtable


@pytest.fixture(scope="module")
def airtable_cached():
    return Airtable(cache_seconds=60)


@pytest.fixture(scope="module")
def airtable_uncached():
    return Airtable()


def new_primary_dict(fields=None):
    return {"fields": fields or {}}


def new_secondary_dict(fields=None):
    return {"fields": fields or {}}


class TestCacheHitReferenceEquality:
    id: str

    def test_create(self, airtable_cached):
        record = new_primary_dict({"Primary Key": "Cache Hit Test"})
        created = airtable_cached.primary.dict.create(record)
        self.__class__.id = created["id"]
        assert created["id"]

    def test_cache_hit_dict(self, airtable_cached):
        first = airtable_cached.primary.dict.get(self.id)
        second = airtable_cached.primary.dict.get(self.id)
        assert first is second

    def test_cache_hit_orm(self, airtable_cached):
        first = airtable_cached.primary.get(self.id)
        second = airtable_cached.primary.get(self.id)
        assert first is second

    def test_cleanup(self, airtable_cached):
        airtable_cached.primary.dict.delete(record_id=self.id)


class TestCacheDisabledByDefault:
    def test_cache_seconds_is_zero(self, airtable_uncached):
        assert airtable_uncached._cache_seconds == 0


class TestMutationInvalidation:
    id: str
    id2: str

    def test_create_record(self, airtable_cached):
        record = new_primary_dict({"Primary Key": "Mutation Test"})
        created = airtable_cached.primary.dict.create(record)
        self.__class__.id = created["id"]

    def test_get_populates_cache(self, airtable_cached):
        airtable_cached.primary.dict.get(self.id)
        assert len(airtable_cached.primary.dict._cache) > 0

    def test_create_invalidates_cache(self, airtable_cached):
        record2 = new_primary_dict({"Primary Key": "Mutation Test 2"})
        created2 = airtable_cached.primary.dict.create(record2)
        self.__class__.id2 = created2["id"]
        assert len(airtable_cached.primary.dict._cache) == 0

    def test_update_invalidates_cache(self, airtable_cached):
        airtable_cached.primary.dict.get(self.id)
        assert len(airtable_cached.primary.dict._cache) > 0
        r = airtable_cached.primary.dict.get(self.id)
        r["fields"]["Primary Key"] = "Mutation Updated"
        airtable_cached.primary.dict.update(r)
        assert len(airtable_cached.primary.dict._cache) == 0

    def test_delete_invalidates_cache(self, airtable_cached):
        airtable_cached.primary.dict.get(self.id)
        assert len(airtable_cached.primary.dict._cache) > 0
        airtable_cached.primary.dict.delete(record_id=self.id2)
        assert len(airtable_cached.primary.dict._cache) == 0

    def test_cleanup(self, airtable_cached):
        airtable_cached.primary.dict.delete(record_id=self.id)


class TestManualInvalidation:
    id: str

    def test_create(self, airtable_cached):
        record = new_primary_dict({"Primary Key": "Manual Invalidation Test"})
        created = airtable_cached.primary.dict.create(record)
        self.__class__.id = created["id"]

    def test_populate_cache(self, airtable_cached):
        airtable_cached.primary.dict.get(self.id)
        assert len(airtable_cached.primary.dict._cache) > 0

    def test_invalidate_clears_cache(self, airtable_cached):
        airtable_cached.primary.invalidate_cache()
        assert len(airtable_cached.primary._cache) == 0
        assert len(airtable_cached.primary.dict._cache) == 0

    def test_get_still_works(self, airtable_cached):
        result = airtable_cached.primary.dict.get(self.id)
        assert result["id"] == self.id

    def test_cleanup(self, airtable_cached):
        airtable_cached.primary.dict.delete(record_id=self.id)


class TestMainClientCascadeInvalidation:
    primary_id: str
    secondary_id: str

    def test_create_records(self, airtable_cached):
        p = new_primary_dict({"Primary Key": "Cascade Primary"})
        created_p = airtable_cached.primary.dict.create(p)
        self.__class__.primary_id = created_p["id"]

        s = new_secondary_dict({"Name": "Cascade Secondary"})
        created_s = airtable_cached.secondary.dict.create(s)
        self.__class__.secondary_id = created_s["id"]

    def test_populate_caches(self, airtable_cached):
        airtable_cached.primary.dict.get(self.primary_id)
        airtable_cached.secondary.dict.get(self.secondary_id)
        assert len(airtable_cached.primary.dict._cache) > 0
        assert len(airtable_cached.secondary.dict._cache) > 0

    def test_cascade_invalidation(self, airtable_cached):
        airtable_cached.invalidate_cache()
        assert len(airtable_cached.primary._cache) == 0
        assert len(airtable_cached.primary.dict._cache) == 0
        assert len(airtable_cached.secondary._cache) == 0
        assert len(airtable_cached.secondary.dict._cache) == 0

    def test_cleanup(self, airtable_cached):
        airtable_cached.primary.dict.delete(record_id=self.primary_id)
        airtable_cached.secondary.dict.delete(record_id=self.secondary_id)


class TestCacheExpiry:
    id: str
    original_cache_seconds: int

    def test_create(self, airtable_cached):
        record = new_primary_dict({"Primary Key": "Expiry Test"})
        created = airtable_cached.primary.dict.create(record)
        self.__class__.id = created["id"]

    def test_cache_then_expire(self, airtable_cached):
        # Temporarily set a short TTL
        self.__class__.original_cache_seconds = airtable_cached.primary.dict._cache_seconds
        airtable_cached.primary.dict._cache_seconds = 2

        first = airtable_cached.primary.dict.get(self.id)
        cached = airtable_cached.primary.dict.get(self.id)
        assert first is cached

        time.sleep(3)

        after_expiry = airtable_cached.primary.dict.get(self.id)
        assert after_expiry is not first

        # Restore original TTL
        airtable_cached.primary.dict._cache_seconds = self.original_cache_seconds

    def test_cleanup(self, airtable_cached):
        airtable_cached.primary.dict.delete(record_id=self.id)


class TestCacheServesStaleData:
    id: str

    def test_create(self, airtable_cached):
        record = new_primary_dict({"Primary Key": "Stale Data Test"})
        created = airtable_cached.primary.dict.create(record)
        self.__class__.id = created["id"]

    def test_stale_data(self, airtable_cached):
        # Populate cache with original value
        first = airtable_cached.primary.dict.get(self.id)
        assert first["fields"]["Primary Key"] == "Stale Data Test"

        # Update via underlying pyairtable API directly (bypasses cache invalidation)
        airtable_cached.primary._table.update(self.id, {"Primary Key": "Updated Directly"})

        # Cached instance should return stale data
        stale = airtable_cached.primary.dict.get(self.id)
        assert stale["fields"]["Primary Key"] == "Stale Data Test"

    def test_cleanup(self, airtable_cached):
        airtable_cached.primary.dict.delete(record_id=self.id)


class TestDifferentParamsDifferentCacheKeys:
    id1: str
    id2: str

    def test_create(self, airtable_cached):
        r1 = new_primary_dict({"Primary Key": "Cache Key 1"})
        r2 = new_primary_dict({"Primary Key": "Cache Key 2"})
        c1 = airtable_cached.primary.dict.create(r1)
        c2 = airtable_cached.primary.dict.create(r2)
        self.__class__.id1 = c1["id"]
        self.__class__.id2 = c2["id"]

    def test_different_keys(self, airtable_cached):
        airtable_cached.primary.invalidate_cache()
        get1 = airtable_cached.primary.dict.get(self.id1)
        get2 = airtable_cached.primary.dict.get(self.id2)
        assert len(airtable_cached.primary.dict._cache) == 2

        re_get1 = airtable_cached.primary.dict.get(self.id1)
        re_get2 = airtable_cached.primary.dict.get(self.id2)
        assert re_get1 is get1
        assert re_get2 is get2

    def test_cleanup(self, airtable_cached):
        airtable_cached.primary.dict.delete(record_id=self.id1)
        airtable_cached.primary.dict.delete(record_id=self.id2)


class TestBatchGetCaching:
    id1: str
    id2: str

    def test_create(self, airtable_cached):
        r1 = new_primary_dict({"Primary Key": "Batch Cache 1"})
        r2 = new_primary_dict({"Primary Key": "Batch Cache 2"})
        c1 = airtable_cached.primary.dict.create(r1)
        c2 = airtable_cached.primary.dict.create(r2)
        self.__class__.id1 = c1["id"]
        self.__class__.id2 = c2["id"]

    def test_batch_cached_separately_from_single(self, airtable_cached):
        airtable_cached.primary.invalidate_cache()

        # Batch get caches as one entry
        batch = airtable_cached.primary.dict.get([self.id1, self.id2])
        assert len(airtable_cached.primary.dict._cache) == 1

        # Single get creates a separate cache entry
        single = airtable_cached.primary.dict.get(self.id1)
        assert len(airtable_cached.primary.dict._cache) == 2

        # Both return cached results on re-get
        batch2 = airtable_cached.primary.dict.get([self.id1, self.id2])
        single2 = airtable_cached.primary.dict.get(self.id1)
        assert batch2 is batch
        assert single2 is single

    def test_cleanup(self, airtable_cached):
        airtable_cached.primary.dict.delete(record_id=self.id1)
        airtable_cached.primary.dict.delete(record_id=self.id2)


class TestFieldsOptionAffectsCacheKey:
    id: str

    def test_create(self, airtable_cached):
        record = new_primary_dict({"Primary Key": "Fields Cache Test", "Single Line Text": "Hello"})
        created = airtable_cached.primary.dict.create(record)
        self.__class__.id = created["id"]

    def test_different_fields_different_cache_entries(self, airtable_cached):
        airtable_cached.primary.invalidate_cache()

        # Get with no fields filter
        full = airtable_cached.primary.dict.get(self.id)
        assert len(airtable_cached.primary.dict._cache) == 1

        # Get with fields filter creates a separate cache entry
        partial = airtable_cached.primary.dict.get(self.id, fields=["Primary Key"])
        assert len(airtable_cached.primary.dict._cache) == 2

        # Both return cached results on re-get
        full2 = airtable_cached.primary.dict.get(self.id)
        partial2 = airtable_cached.primary.dict.get(self.id, fields=["Primary Key"])
        assert full2 is full
        assert partial2 is partial

        # Partial result should not have the extra field
        assert "Single Line Text" not in partial["fields"]

    def test_cleanup(self, airtable_cached):
        airtable_cached.primary.dict.delete(record_id=self.id)


class TestFormulaQueryCaching:
    id: str

    def test_create(self, airtable_cached):
        record = new_primary_dict({"Primary Key": "Formula Cache Test"})
        created = airtable_cached.primary.dict.create(record)
        self.__class__.id = created["id"]

    def test_formula_query_is_cached(self, airtable_cached):
        airtable_cached.primary.invalidate_cache()

        formula = "{Primary Key} = 'Formula Cache Test'"
        first = airtable_cached.primary.dict.get(formula=formula)
        assert len(airtable_cached.primary.dict._cache) == 1

        second = airtable_cached.primary.dict.get(formula=formula)
        assert first is second

    def test_different_formulas_different_cache_entries(self, airtable_cached):
        airtable_cached.primary.invalidate_cache()

        f1 = "{Primary Key} = 'Formula Cache Test'"
        f2 = "{Primary Key} != 'nonexistent'"
        airtable_cached.primary.dict.get(formula=f1)
        airtable_cached.primary.dict.get(formula=f2)
        assert len(airtable_cached.primary.dict._cache) == 2

    def test_cleanup(self, airtable_cached):
        airtable_cached.primary.dict.delete(record_id=self.id)


class TestOrmAndDictIndependentCaches:
    id: str

    def test_create(self, airtable_cached):
        record = new_primary_dict({"Primary Key": "Independent Cache Test"})
        created = airtable_cached.primary.dict.create(record)
        self.__class__.id = created["id"]

    def test_orm_cache_independent_of_dict(self, airtable_cached):
        airtable_cached.primary.invalidate_cache()

        # ORM get populates ORM cache only
        airtable_cached.primary.get(self.id)
        assert len(airtable_cached.primary._cache) > 0
        assert len(airtable_cached.primary.dict._cache) == 0

        # Dict get populates dict cache only
        airtable_cached.primary.dict.get(self.id)
        assert len(airtable_cached.primary._cache) > 0
        assert len(airtable_cached.primary.dict._cache) > 0

    def test_cleanup(self, airtable_cached):
        airtable_cached.primary.dict.delete(record_id=self.id)
