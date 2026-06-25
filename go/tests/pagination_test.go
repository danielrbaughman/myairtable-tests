package tests

// TC1 — Multi-page pagination: list more records than Airtable's 100-record
// default page in a single query (no maxRecords) and assert the client's offset
// loop reassembles every page. Exercises the page-reassembly path in the
// generated client. Parity with the C# TestPagination and the other suites.

import (
	"context"
	"fmt"
	"testing"

	airtable "myairtabletests/output"
)

// 105 > the 100-record default page size, so a no-maxRecords list spans two pages.
const paginationCount = 105

// TC5 — explicit page size. 25 records over pageSize 10 spans 3 pages (10+10+5).
const pageSizeCount = 25

func TestPagination(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	// findFilter restricts a list to records whose primary key contains the
	// per-run-unique suite prefix.
	findFilter := func(suite string) string {
		return fmt.Sprintf("FIND(\"%s\", {Primary Key})", suite)
	}

	// tryDeleteMany cleans up created records best-effort, falling back to a
	// prefix sweep (list by FIND on the suite prefix and delete) on failure.
	tryDeleteMany := func(ids []string, suite string) {
		if len(ids) > 0 {
			if err := at.Primary.Dict().DeleteMany(ctx, ids); err == nil {
				return
			}
		}
		stray, err := at.Primary.Dict().GetMany(ctx, (&airtable.Query{}).WithFilter(findFilter(suite)))
		if err != nil || len(stray) == 0 {
			return
		}
		strayIDs := make([]string, 0, len(stray))
		for _, r := range stray {
			strayIDs = append(strayIDs, r.ID)
		}
		_ = at.Primary.Dict().DeleteMany(ctx, strayIDs)
	}

	t.Run("OrmGetSpanningMultiplePagesReturnsEveryRecord", func(t *testing.T) {
		suite := primaryKey("Pagination", "Orm")
		models := make([]*airtable.PrimaryModel, paginationCount)
		for i := range models {
			models[i] = &airtable.PrimaryModel{PrimaryKey: airtable.String(fmt.Sprintf("%s %d", suite, i+1))}
		}

		var createdIDs []string
		defer func() { tryDeleteMany(createdIDs, suite) }()

		created, err := at.Primary.CreateMany(ctx, models)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		createdIDs = make([]string, 0, len(created))
		for _, c := range created {
			createdIDs = append(createdIDs, c.ID())
		}
		if len(created) != paginationCount {
			t.Fatalf("expected %d created, got %d", paginationCount, len(created))
		}

		// No maxRecords: the offset loop must walk both pages and return all 105.
		results, err := at.Primary.GetMany(ctx, (&airtable.Query{}).WithFilter(findFilter(suite)))
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		if len(results) != paginationCount {
			t.Fatalf("expected %d results, got %d", paginationCount, len(results))
		}
		resultIDs := make([]string, 0, len(results))
		for _, r := range results {
			resultIDs = append(resultIDs, r.ID())
		}
		if !sameIDSet(createdIDs, resultIDs) {
			t.Fatalf("returned id set does not match created id set")
		}
	})

	t.Run("DictGetSpanningMultiplePagesReturnsEveryRecord", func(t *testing.T) {
		suite := primaryKey("Pagination", "Dict")
		creates := make([]*airtable.Fields, paginationCount)
		for i := range creates {
			f := airtable.NewFields(nil, airtable.PrimaryNameToID)
			if err := f.Set(airtable.PrimaryPrimaryKeyFieldID, fmt.Sprintf("%s %d", suite, i+1)); err != nil {
				t.Fatalf("set primary key: %v", err)
			}
			creates[i] = f
		}

		var createdIDs []string
		defer func() { tryDeleteMany(createdIDs, suite) }()

		created, err := at.Primary.Dict().CreateMany(ctx, creates)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		createdIDs = make([]string, 0, len(created))
		for _, c := range created {
			createdIDs = append(createdIDs, c.ID)
		}
		if len(created) != paginationCount {
			t.Fatalf("expected %d created, got %d", paginationCount, len(created))
		}

		results, err := at.Primary.Dict().GetMany(ctx, (&airtable.Query{}).WithFilter(findFilter(suite)))
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		if len(results) != paginationCount {
			t.Fatalf("expected %d results, got %d", paginationCount, len(results))
		}
		resultIDs := make([]string, 0, len(results))
		for _, r := range results {
			resultIDs = append(resultIDs, r.ID)
		}
		if !sameIDSet(createdIDs, resultIDs) {
			t.Fatalf("returned id set does not match created id set")
		}
	})

	// TC5 — explicit page size: WithPageSize(10) over 25 records, NO maxRecords. The
	// offset loop must reassemble 3 pages (10+10+5) and return all 25.
	t.Run("ExplicitPageSizeReturnsAllRecordsAcrossPages", func(t *testing.T) {
		suite := primaryKey("Pagination", "PageSize")
		models := make([]*airtable.PrimaryModel, pageSizeCount)
		for i := range models {
			models[i] = &airtable.PrimaryModel{PrimaryKey: airtable.String(fmt.Sprintf("%s %d", suite, i+1))}
		}

		var createdIDs []string
		defer func() { tryDeleteMany(createdIDs, suite) }()

		created, err := at.Primary.CreateMany(ctx, models)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		createdIDs = make([]string, 0, len(created))
		for _, c := range created {
			createdIDs = append(createdIDs, c.ID())
		}

		// pageSize 10, no maxRecords: the offset loop must walk all 3 pages and return all 25.
		results, err := at.Primary.GetMany(ctx, (&airtable.Query{}).WithFilter(findFilter(suite)).WithPageSize(10))
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		if len(results) != pageSizeCount {
			t.Fatalf("expected %d results, got %d", pageSizeCount, len(results))
		}
	})

	// TC5 — explicit page size + maxRecords: WithPageSize(10) AND WithMaxRecords(15)
	// over 25 records. maxRecords caps the total mid-stream (not a page multiple).
	t.Run("PageSizeWithMaxRecordsCapsTheTotal", func(t *testing.T) {
		suite := primaryKey("Pagination", "PageCap")
		models := make([]*airtable.PrimaryModel, pageSizeCount)
		for i := range models {
			models[i] = &airtable.PrimaryModel{PrimaryKey: airtable.String(fmt.Sprintf("%s %d", suite, i+1))}
		}

		var createdIDs []string
		defer func() { tryDeleteMany(createdIDs, suite) }()

		created, err := at.Primary.CreateMany(ctx, models)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		createdIDs = make([]string, 0, len(created))
		for _, c := range created {
			createdIDs = append(createdIDs, c.ID())
		}

		// pageSize 10 + maxRecords 15: maxRecords caps the total mid-stream.
		results, err := at.Primary.GetMany(ctx, (&airtable.Query{}).WithFilter(findFilter(suite)).WithPageSize(10).WithMaxRecords(15))
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		if len(results) != 15 {
			t.Fatalf("expected %d results, got %d", 15, len(results))
		}
	})
}

// sameIDSet reports whether a and b contain exactly the same set of ids.
func sameIDSet(a, b []string) bool {
	if len(a) != len(b) {
		return false
	}
	set := make(map[string]bool, len(a))
	for _, id := range a {
		set[id] = true
	}
	for _, id := range b {
		if !set[id] {
			return false
		}
	}
	return true
}
