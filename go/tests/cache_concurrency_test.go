package tests

// TC9 — Cache thread-safety. The TTL cache (cacheStore) is shared mutable state
// on the client; the caching suite only exercises it single-threaded. These fire
// concurrent reads and concurrent mutations at one cached client and assert no
// corruption/panic and a consistent result.
//
// CRITICAL: run this suite under the race detector — an unsynchronized cache map
// would be a real data race:
//
//	go test -race -run TestCacheConcurrency ./...
//
// Parity with csharp/tests/TestCacheConcurrency.cs.

import (
	"context"
	"sync"
	"testing"

	airtable "myairtabletests/output"
)

func TestCacheConcurrency(t *testing.T) {
	requireCreds(t)
	ctx := context.Background()

	t.Run("ConcurrentGetsOfTheSameRecordAreConsistent", func(t *testing.T) {
		at := cachedAirtable(t, 60)
		key := primaryKey("CacheConc", "Reads")
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey: airtable.String(key),
		})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		id := created.ID()
		defer bestEffortDelete(at, id)

		// 25 concurrent gets race to populate/read the shared cache.
		const n = 25
		var wg sync.WaitGroup
		results := make([]*airtable.PrimaryModel, n)
		errs := make([]error, n)
		for i := 0; i < n; i++ {
			wg.Add(1)
			go func(idx int) {
				defer wg.Done()
				results[idx], errs[idx] = at.Primary.GetOne(ctx, id)
			}(i)
		}
		wg.Wait()

		for i := 0; i < n; i++ {
			if errs[i] != nil {
				t.Fatalf("concurrent get %d: %v", i, errs[i])
			}
			m := results[i]
			if m == nil {
				t.Fatalf("concurrent get %d returned nil model", i)
			}
			if m.PrimaryKey == nil || *m.PrimaryKey != key {
				t.Fatalf("concurrent get %d PrimaryKey = %v, want %q", i, m.PrimaryKey, key)
			}
			if m.ID() != id {
				t.Fatalf("concurrent get %d id = %s, want %s", i, m.ID(), id)
			}
		}
	})

	t.Run("ConcurrentReadsAndMutationsDoNotCorruptTheCache", func(t *testing.T) {
		at := cachedAirtable(t, 60)
		key := primaryKey("CacheConc", "Mix")
		created, err := at.Primary.CreateOne(ctx, &airtable.PrimaryModel{
			PrimaryKey: airtable.String(key),
		})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		id := created.ID()
		defer bestEffortDelete(at, id)

		tableID := (&airtable.PrimaryModel{}).TableID()

		// Interleave reads (populate cache), per-table invalidations, and
		// invalidate-all concurrently ~15x.
		const rounds = 15
		var wg sync.WaitGroup
		errs := make([]error, rounds)
		for i := 0; i < rounds; i++ {
			wg.Add(3)
			go func(idx int) {
				defer wg.Done()
				_, errs[idx] = at.Primary.GetOne(ctx, id)
			}(i)
			go func() {
				defer wg.Done()
				at.Client().InvalidateTableCache(tableID)
			}()
			go func() {
				defer wg.Done()
				at.InvalidateAllCaches()
			}()
		}
		wg.Wait()

		for i, e := range errs {
			if e != nil {
				t.Fatalf("interleaved get %d: %v", i, e)
			}
		}

		// The client is still usable and returns the correct value afterward.
		fetched, err := at.Primary.GetOne(ctx, id)
		if err != nil {
			t.Fatalf("final get: %v", err)
		}
		if fetched.PrimaryKey == nil || *fetched.PrimaryKey != key {
			t.Fatalf("final PrimaryKey = %v, want %q", fetched.PrimaryKey, key)
		}
	})
}
