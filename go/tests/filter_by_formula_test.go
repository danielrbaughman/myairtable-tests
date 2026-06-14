package tests

// Feature 7 — filter-by-formula live integration tests. Parity with Java
// TestFilterByFormula / Kotlin / Swift. Live against the real base; each subtest
// uses distinct primary keys and cleans up its records.

import (
	"context"
	"sort"
	"strconv"
	"strings"
	"testing"

	airtable "myairtabletests/output"
)

// scopeTo restricts a formula to specific record IDs so concurrent suites and
// reruns never interfere.
func scopeTo(ids []string) airtable.Formula {
	parts := make([]airtable.Formula, 0, len(ids))
	for _, id := range ids {
		parts = append(parts, airtable.PrimaryF.ID.Eq(id))
	}
	if len(parts) == 1 {
		return parts[0]
	}
	return airtable.Or(parts...)
}

// fbPrimaryFields builds a Primary dict Fields container with just the primary key.
func fbPrimaryFields(t *testing.T, name string) *airtable.Fields {
	t.Helper()
	f := airtable.NewFields(nil, airtable.PrimaryNameToID)
	if err := f.Set(airtable.PrimaryPrimaryKeyFieldID, name); err != nil {
		t.Fatalf("set primary key: %v", err)
	}
	return f
}

func fbNames(t *testing.T, recs []*airtable.DictRecord) []string {
	t.Helper()
	out := make([]string, 0, len(recs))
	for _, r := range recs {
		var s string
		if _, ok := r.Fields.GetRaw(airtable.PrimaryPrimaryKeyFieldID); ok {
			if err := r.Fields.Get(airtable.PrimaryPrimaryKeyFieldID, &s); err == nil && s != "" {
				out = append(out, s)
			}
		}
	}
	sort.Strings(out)
	return out
}

func TestFilterByFormula(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	// fbCount returns the number of Primary dict records matching formula within scope.
	fbCount := func(scope, formula airtable.Formula) int {
		recs, err := at.PrimaryDict.List(ctx, (&airtable.Query{}).WithFilterFormula(airtable.And(scope, formula)))
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		return len(recs)
	}

	// fbMatchNames returns sorted primary-key names matching formula within scope.
	fbMatchNames := func(scope, formula airtable.Formula) []string {
		recs, err := at.PrimaryDict.List(ctx, (&airtable.Query{}).WithFilterFormula(airtable.And(scope, formula)))
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		return fbNames(t, recs)
	}

	f := airtable.PrimaryF

	t.Run("TextFieldEquals", func(t *testing.T) {
		pk := primaryKey("Filter", "TextEq")
		created, err := at.Primary.Create(ctx, &airtable.PrimaryModel{
			PrimaryKey:     airtable.String(pk),
			SingleLineText: airtable.String("UniqueFilterValue"),
		})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		defer at.Primary.Delete(ctx, created.ID()) //nolint:errcheck

		rows, err := at.Primary.List(ctx, (&airtable.Query{}).WithFilterFormula(f.SingleLineText.Eq("UniqueFilterValue", true, false)))
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		if !containsID(rows, created.ID()) {
			t.Fatal("expected created record in results")
		}
	})

	t.Run("RecordIDFilter", func(t *testing.T) {
		pk := primaryKey("Filter", "RecordID")
		created, err := at.Primary.Create(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(pk)})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		defer at.Primary.Delete(ctx, created.ID()) //nolint:errcheck

		rows, err := at.Primary.List(ctx, (&airtable.Query{}).WithFilterFormula(f.ID.Eq(created.ID())))
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		if len(rows) != 1 || rows[0].ID() != created.ID() {
			t.Fatalf("expected exactly the created record, got %d", len(rows))
		}
	})

	t.Run("RecordIDInList", func(t *testing.T) {
		suite := primaryKey("Filter", "InList")
		fieldsList := []*airtable.Fields{
			fbPrimaryFields(t, suite+" 0"),
			fbPrimaryFields(t, suite+" 1"),
			fbPrimaryFields(t, suite+" 2"),
		}
		created, err := at.PrimaryDict.CreateMany(ctx, fieldsList)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		ids := dictIDs(created)
		defer at.PrimaryDict.DeleteMany(ctx, ids) //nolint:errcheck

		recs, err := at.PrimaryDict.List(ctx, (&airtable.Query{}).WithFilterFormula(f.ID.InList([]string{ids[0], ids[1]})))
		if err != nil {
			t.Fatalf("list multi: %v", err)
		}
		if len(recs) != 2 {
			t.Fatalf("inList multi: expected 2, got %d", len(recs))
		}

		recs, err = at.PrimaryDict.List(ctx, (&airtable.Query{}).WithFilterFormula(f.ID.InList([]string{ids[0]})))
		if err != nil {
			t.Fatalf("list single: %v", err)
		}
		if len(recs) != 1 {
			t.Fatalf("inList single: expected 1, got %d", len(recs))
		}

		recs, err = at.PrimaryDict.List(ctx, (&airtable.Query{}).WithFilterFormula(f.ID.InList(nil)))
		if err != nil {
			t.Fatalf("list empty: %v", err)
		}
		if len(recs) != 0 {
			t.Fatalf("inList empty: expected 0, got %d", len(recs))
		}
	})

	t.Run("NumberFieldFilter", func(t *testing.T) {
		suite := primaryKey("Filter", "Number")
		c1, err := at.Primary.Create(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(suite + " 1"), NumberInt: airtable.Float64(10)})
		if err != nil {
			t.Fatalf("create 1: %v", err)
		}
		defer at.Primary.Delete(ctx, c1.ID()) //nolint:errcheck
		c2, err := at.Primary.Create(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(suite + " 2"), NumberInt: airtable.Float64(20)})
		if err != nil {
			t.Fatalf("create 2: %v", err)
		}
		defer at.Primary.Delete(ctx, c2.ID()) //nolint:errcheck

		rows, err := at.Primary.List(ctx, (&airtable.Query{}).WithFilterFormula(f.NumberInt.GreaterThan(15)))
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		if !containsID(rows, c2.ID()) || containsID(rows, c1.ID()) {
			t.Fatal("greaterThan(15) should match only c2")
		}
	})

	t.Run("TextFieldPredicates", func(t *testing.T) {
		fieldsList := []*airtable.Fields{
			fbPrimaryFields(t, "FbText Alpha One"),
			fbPrimaryFields(t, "FbText Alpha Two"),
			fbPrimaryFields(t, "FbText Beta One"),
		}
		created, err := at.PrimaryDict.CreateMany(ctx, fieldsList)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		ids := dictIDs(created)
		defer at.PrimaryDict.DeleteMany(ctx, ids) //nolint:errcheck
		scope := scopeTo(ids)

		checks := []struct {
			label string
			want  int
			got   int
		}{
			{"eq", 1, fbCount(scope, f.PrimaryKey.Eq("FbText Alpha One", true, false))},
			{"neq", 2, fbCount(scope, f.PrimaryKey.Neq("FbText Alpha One", true, false))},
			{"contains", 2, fbCount(scope, f.PrimaryKey.Contains("Alpha", false, true))},
			{"containsAny", 3, fbCount(scope, f.PrimaryKey.ContainsAny([]string{"Alpha", "Beta"}, false, true))},
			{"containsAll", 1, fbCount(scope, f.PrimaryKey.ContainsAll([]string{"Alpha", "One"}, false, true))},
			{"notContains", 1, fbCount(scope, f.PrimaryKey.NotContains("Alpha", false, true))},
			{"startsWith", 2, fbCount(scope, f.PrimaryKey.StartsWith("FbText Alpha", false, true))},
			{"notStartsWith", 1, fbCount(scope, f.PrimaryKey.NotStartsWith("FbText Alpha", false, true))},
		}
		for _, c := range checks {
			if c.got != c.want {
				t.Errorf("%s: got %d, want %d", c.label, c.got, c.want)
			}
		}
	})

	t.Run("NumberFieldPredicates", func(t *testing.T) {
		suite := primaryKey("Filter", "NumPreds")
		values := []float64{10, 20, 30}
		fieldsList := make([]*airtable.Fields, 0, len(values))
		for i, v := range values {
			ff := fbPrimaryFields(t, suite+" "+itoa(i))
			if err := ff.Set(airtable.PrimaryNumberIntFieldID, v); err != nil {
				t.Fatalf("set number: %v", err)
			}
			fieldsList = append(fieldsList, ff)
		}
		created, err := at.PrimaryDict.CreateMany(ctx, fieldsList)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		ids := dictIDs(created)
		defer at.PrimaryDict.DeleteMany(ctx, ids) //nolint:errcheck
		scope := scopeTo(ids)

		checks := []struct {
			label string
			want  int
			got   int
		}{
			{"eq", 1, fbCount(scope, f.NumberInt.Eq(20))},
			{"neq", 2, fbCount(scope, f.NumberInt.Neq(20))},
			{"greaterThan", 2, fbCount(scope, f.NumberInt.GreaterThan(10))},
			{"lessThan", 2, fbCount(scope, f.NumberInt.LessThan(30))},
			{"gte", 2, fbCount(scope, f.NumberInt.GreaterThanOrEqual(20))},
			{"lte", 2, fbCount(scope, f.NumberInt.LessThanOrEqual(20))},
			{"betweenInclusive", 3, fbCount(scope, f.NumberInt.Between(10, 30, true))},
			{"betweenExclusive", 1, fbCount(scope, f.NumberInt.Between(10, 30, false))},
		}
		for _, c := range checks {
			if c.got != c.want {
				t.Errorf("%s: got %d, want %d", c.label, c.got, c.want)
			}
		}
	})

	t.Run("BooleanFieldFilter", func(t *testing.T) {
		suite := primaryKey("Filter", "Bool")
		f1 := fbPrimaryFields(t, suite+" A")
		_ = f1.Set(airtable.PrimaryCheckboxFieldID, true)
		f2 := fbPrimaryFields(t, suite+" B")
		_ = f2.Set(airtable.PrimaryCheckboxFieldID, false)
		created, err := at.PrimaryDict.CreateMany(ctx, []*airtable.Fields{f1, f2})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		ids := dictIDs(created)
		defer at.PrimaryDict.DeleteMany(ctx, ids) //nolint:errcheck
		scope := scopeTo(ids)

		if got := fbCount(scope, f.Checkbox.Eq(true)); got != 1 {
			t.Errorf("eq(true): got %d, want 1", got)
		}
		if got := fbCount(scope, f.Checkbox.IsTrue()); got != 1 {
			t.Errorf("isTrue: got %d, want 1", got)
		}
		if got := fbCount(scope, f.Checkbox.IsFalse()); got < 1 {
			t.Errorf("isFalse: got %d, want >=1", got)
		}
	})

	t.Run("AttachmentsFieldFilter", func(t *testing.T) {
		suite := primaryKey("Filter", "Attach")
		f1 := fbPrimaryFields(t, suite+" A")
		attachments := []map[string]string{{"url": "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"}}
		if err := f1.Set(airtable.PrimaryAttachmentFieldID, attachments); err != nil {
			t.Fatalf("set attachment: %v", err)
		}
		f2 := fbPrimaryFields(t, suite+" B")
		created, err := at.PrimaryDict.CreateMany(ctx, []*airtable.Fields{f1, f2})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		ids := dictIDs(created)
		defer at.PrimaryDict.DeleteMany(ctx, ids) //nolint:errcheck
		scope := scopeTo(ids)

		if got := fbCount(scope, f.Attachment.IsNotEmpty()); got < 1 {
			t.Errorf("notEmpty: got %d, want >=1", got)
		}
		if got := fbCount(scope, f.Attachment.IsEmpty()); got < 1 {
			t.Errorf("empty: got %d, want >=1", got)
		}
	})

	t.Run("DateFieldPredicates", func(t *testing.T) {
		suite := primaryKey("Filter", "Date")
		f1 := fbPrimaryFields(t, suite+" A")
		_ = f1.Set(airtable.PrimaryDateFieldID, "2024-01-15")
		f2 := fbPrimaryFields(t, suite+" B")
		_ = f2.Set(airtable.PrimaryDateFieldID, "2024-06-15")
		f3 := fbPrimaryFields(t, suite+" C") // no date
		created, err := at.PrimaryDict.CreateMany(ctx, []*airtable.Fields{f1, f2, f3})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		ids := dictIDs(created)
		defer at.PrimaryDict.DeleteMany(ctx, ids) //nolint:errcheck
		scope := scopeTo(ids)

		if got := fbCount(scope, f.Date.OnDate("2024-01-15")); got != 1 {
			t.Errorf("on: got %d, want 1", got)
		}
		if got := fbCount(scope, f.Date.NotOnDate("2024-01-15")); got < 1 {
			t.Errorf("notOn: got %d, want >=1", got)
		}
		if got := fbCount(scope, f.Date.OnOrAfterDate("2024-06-15")); got < 1 {
			t.Errorf("onOrAfter: got %d, want >=1", got)
		}
		if got := fbCount(scope, f.Date.OnOrBeforeDate("2024-01-15")); got < 1 {
			t.Errorf("onOrBefore: got %d, want >=1", got)
		}
		if got := fbCount(scope, f.Date.AfterDate("2024-01-15")); got < 1 {
			t.Errorf("after: got %d, want >=1", got)
		}
		if got := fbCount(scope, f.Date.BeforeDate("2024-06-15")); got < 1 {
			t.Errorf("before: got %d, want >=1", got)
		}
		if got := fbCount(scope, f.Date.BetweenDates("2024-01-15", "2024-06-15", true)); got != 2 {
			t.Errorf("between incl: got %d, want 2", got)
		}
		if got := fbCount(scope, f.Date.BetweenDates("2024-01-01", "2024-12-31", false)); got != 2 {
			t.Errorf("between excl: got %d, want 2", got)
		}
		if got := fbCount(scope, f.Date.IsNotEmpty()); got < 1 {
			t.Errorf("notEmpty: got %d, want >=1", got)
		}
		if got := fbCount(scope, f.Date.IsEmpty()); got < 1 {
			t.Errorf("empty: got %d, want >=1", got)
		}
	})

	t.Run("DateFieldChained", func(t *testing.T) {
		suite := primaryKey("Filter", "DateChain")
		f1 := fbPrimaryFields(t, suite+" A")
		_ = f1.Set(airtable.PrimaryDateFieldID, "2024-01-15")
		f2 := fbPrimaryFields(t, suite+" B")
		_ = f2.Set(airtable.PrimaryDateFieldID, "2024-06-15")
		created, err := at.PrimaryDict.CreateMany(ctx, []*airtable.Fields{f1, f2})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		ids := dictIDs(created)
		defer at.PrimaryDict.DeleteMany(ctx, ids) //nolint:errcheck
		scope := scopeTo(ids)

		// before().daysAgo(1) — both dates are in the past.
		if got := fbCount(scope, f.Date.BeforeChain().DaysAgo(1)); got < 1 {
			t.Errorf("before daysAgo(1): got %d, want >=1", got)
		}
		// after().yearsAgo(100) — both dates within the last 100 years.
		if got := fbCount(scope, f.Date.AfterChain().YearsAgo(100)); got < 1 {
			t.Errorf("after yearsAgo(100): got %d, want >=1", got)
		}
	})

	t.Run("ComplexFormulas", func(t *testing.T) {
		suite := primaryKey("Filter", "Complex")
		f1 := fbPrimaryFields(t, suite+" A")
		_ = f1.Set(airtable.PrimaryNumberIntFieldID, float64(10))
		_ = f1.Set(airtable.PrimaryCheckboxFieldID, true)
		f2 := fbPrimaryFields(t, suite+" B")
		_ = f2.Set(airtable.PrimaryNumberIntFieldID, float64(20))
		_ = f2.Set(airtable.PrimaryCheckboxFieldID, false)
		f3 := fbPrimaryFields(t, suite+" C")
		_ = f3.Set(airtable.PrimaryNumberIntFieldID, float64(30))
		_ = f3.Set(airtable.PrimaryCheckboxFieldID, true)
		created, err := at.PrimaryDict.CreateMany(ctx, []*airtable.Fields{f1, f2, f3})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		ids := dictIDs(created)
		defer at.PrimaryDict.DeleteMany(ctx, ids) //nolint:errcheck
		scope := scopeTo(ids)

		checks := []struct {
			label string
			want  int
			got   int
		}{
			{"AND(>10, checkbox) -> C", 1, fbCount(scope, airtable.And(f.NumberInt.GreaterThan(10), f.Checkbox.IsTrue()))},
			{"AND(contains, gte20) -> B,C", 2, fbCount(scope, airtable.And(f.PrimaryKey.Contains(suite, false, true), f.NumberInt.GreaterThanOrEqual(20)))},
			{"OR(=10, =30) -> A,C", 2, fbCount(scope, airtable.Or(f.NumberInt.Eq(10), f.NumberInt.Eq(30)))},
			{"OR(checkbox, =20) -> all", 3, fbCount(scope, airtable.Or(f.Checkbox.IsTrue(), f.NumberInt.Eq(20)))},
			{"XOR(checkbox, >=30) -> A", 1, fbCount(scope, airtable.Xor(f.Checkbox.IsTrue(), f.NumberInt.GreaterThanOrEqual(30)))},
			{"NOT(=A) -> B,C", 2, fbCount(scope, airtable.Not(f.PrimaryKey.Eq(suite+" A", true, false)))},
			{"AND(OR(=10,=30), checkbox) -> A,C", 2, fbCount(scope, airtable.And(airtable.Or(f.NumberInt.Eq(10), f.NumberInt.Eq(30)), f.Checkbox.IsTrue()))},
			{"OR(AND(>10,checkbox), =A) -> A,C", 2, fbCount(scope, airtable.Or(airtable.And(f.NumberInt.GreaterThan(10), f.Checkbox.IsTrue()), f.PrimaryKey.Eq(suite+" A", true, false)))},
			{"NOT(AND(checkbox,>20)) -> A,B", 2, fbCount(scope, airtable.Not(airtable.And(f.Checkbox.IsTrue(), f.NumberInt.GreaterThan(20))))},
			{"AND(NOT(checkbox), gte20) -> B", 1, fbCount(scope, airtable.And(airtable.Not(f.Checkbox.IsTrue()), f.NumberInt.GreaterThanOrEqual(20)))},
		}
		for _, c := range checks {
			if c.got != c.want {
				t.Errorf("%s: got %d, want %d", c.label, c.got, c.want)
			}
		}
	})

	t.Run("FilterByView", func(t *testing.T) {
		fieldsList := make([]*airtable.Fields, 0, 10)
		for i := 0; i < 5; i++ {
			fieldsList = append(fieldsList, fbPrimaryFields(t, "Filter Test "+itoa(i)))
		}
		for i := 0; i < 5; i++ {
			fieldsList = append(fieldsList, fbPrimaryFields(t, "Don't Include Test "+itoa(i)))
		}
		created, err := at.PrimaryDict.CreateMany(ctx, fieldsList)
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		ids := dictIDs(created)
		defer at.PrimaryDict.DeleteMany(ctx, ids) //nolint:errcheck

		recs, err := at.PrimaryDict.List(ctx, (&airtable.Query{}).WithView(airtable.PrimaryViewFilterByView))
		if err != nil {
			t.Fatalf("list: %v", err)
		}
		if len(recs) < 5 {
			t.Fatalf("expected >=5 view records, got %d", len(recs))
		}
		for _, r := range recs {
			var name string
			_ = r.Fields.Get(airtable.PrimaryPrimaryKeyFieldID, &name)
			if !strings.HasPrefix(name, "Filter Test") {
				t.Fatalf("unexpected record in view: %q", name)
			}
		}
	})

	t.Run("LookupFieldFilter", func(t *testing.T) {
		suite := primaryKey("Filter", "Lookup")
		secA, err := at.Secondary.Create(ctx, &airtable.SecondaryModel{Name: airtable.String(suite + " Sec A"), Value: airtable.String("Groundwork BioAg")})
		if err != nil {
			t.Fatalf("create secA: %v", err)
		}
		secB, err := at.Secondary.Create(ctx, &airtable.SecondaryModel{Name: airtable.String(suite + " Sec B"), Value: airtable.String("Groundwork Lab")})
		if err != nil {
			t.Fatalf("create secB: %v", err)
		}
		secC, err := at.Secondary.Create(ctx, &airtable.SecondaryModel{Name: airtable.String(suite + " Sec C"), Value: airtable.String("Other Vendor")})
		if err != nil {
			t.Fatalf("create secC: %v", err)
		}
		secIDs := []string{secA.ID(), secB.ID(), secC.ID()}
		defer at.SecondaryDict.DeleteMany(ctx, secIDs) //nolint:errcheck

		primA, err := at.Primary.Create(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(suite + " A"), LinkSingle: []string{secA.ID()}})
		if err != nil {
			t.Fatalf("create primA: %v", err)
		}
		primB, err := at.Primary.Create(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(suite + " B"), LinkSingle: []string{secB.ID()}})
		if err != nil {
			t.Fatalf("create primB: %v", err)
		}
		primC, err := at.Primary.Create(ctx, &airtable.PrimaryModel{PrimaryKey: airtable.String(suite + " C"), LinkSingle: []string{secC.ID()}})
		if err != nil {
			t.Fatalf("create primC: %v", err)
		}
		primIDs := []string{primA.ID(), primB.ID(), primC.ID()}
		defer at.PrimaryDict.DeleteMany(ctx, primIDs) //nolint:errcheck
		scope := scopeTo(primIDs)

		if got := fbMatchNames(scope, f.Lookup.Contains("groundwork", false, true)); !equalStrings(got, []string{suite + " A", suite + " B"}) {
			t.Errorf("contains: got %v", got)
		}
		if got := fbMatchNames(scope, f.Lookup.Contains("nonexistent-substring-xyz", false, true)); len(got) != 0 {
			t.Errorf("contains no-match: got %v", got)
		}
		if got := fbMatchNames(scope, f.Lookup.StartsWith("Ground", false, true)); !equalStrings(got, []string{suite + " A", suite + " B"}) {
			t.Errorf("startsWith: got %v", got)
		}
		if got := fbMatchNames(scope, f.Lookup.EndsWith("BioAg", false, true)); !equalStrings(got, []string{suite + " A"}) {
			t.Errorf("endsWith: got %v", got)
		}
		if got := fbMatchNames(scope, f.Lookup.Eq("Groundwork BioAg", true, false)); !equalStrings(got, []string{suite + " A"}) {
			t.Errorf("eq: got %v", got)
		}
		if got := fbMatchNames(scope, f.Lookup.NotContains("Groundwork", false, true)); !equalStrings(got, []string{suite + " C"}) {
			t.Errorf("notContains: got %v", got)
		}
	})

	t.Run("DateFieldVsField", func(t *testing.T) {
		suite := primaryKey("Filter", "VsField")
		df := airtable.FormulasF
		mk := func(name, first, second, third string) *airtable.Fields {
			ff := airtable.NewFields(nil, airtable.FormulasNameToID)
			_ = ff.Set(airtable.FormulasPrimaryKeyFieldID, suite+" "+name)
			_ = ff.Set(airtable.FormulasFirstDateFieldID, first)
			_ = ff.Set(airtable.FormulasSecondDateFieldID, second)
			_ = ff.Set(airtable.FormulasThirdDateFieldID, third)
			return ff
		}
		created, err := at.FormulasDict.CreateMany(ctx, []*airtable.Fields{
			mk("Equal", "2024-06-15", "2024-06-15T00:00:00.000Z", "2024-06-15T00:00:00.000Z"),
			mk("FirstBefore", "2024-01-15", "2024-06-15T00:00:00.000Z", "2024-12-15T00:00:00.000Z"),
			mk("FirstAfter", "2024-12-15", "2024-06-15T00:00:00.000Z", "2024-01-15T00:00:00.000Z"),
			mk("Between", "2024-06-15", "2024-01-15T00:00:00.000Z", "2024-12-15T00:00:00.000Z"),
		})
		if err != nil {
			t.Fatalf("create: %v", err)
		}
		ids := dictIDs(created)
		defer at.FormulasDict.DeleteMany(ctx, ids) //nolint:errcheck

		formulasScope := func() airtable.Formula {
			parts := make([]airtable.Formula, 0, len(ids))
			for _, id := range ids {
				parts = append(parts, df.ID.Eq(id))
			}
			return airtable.Or(parts...)
		}()

		matchNames := func(formula airtable.Formula) map[string]bool {
			recs, err := at.FormulasDict.List(ctx, (&airtable.Query{}).WithFilterFormula(airtable.And(formulasScope, formula)))
			if err != nil {
				t.Fatalf("list: %v", err)
			}
			out := map[string]bool{}
			for _, r := range recs {
				var name string
				_ = r.Fields.Get(airtable.FormulasPrimaryKeyFieldID, &name)
				out[strings.TrimPrefix(name, suite+" ")] = true
			}
			return out
		}

		assertSet := func(label string, got map[string]bool, want ...string) {
			if len(got) != len(want) {
				t.Errorf("%s: got %v, want %v", label, got, want)
				return
			}
			for _, w := range want {
				if !got[w] {
					t.Errorf("%s: missing %q in %v", label, w, got)
				}
			}
		}

		assertSet("on", matchNames(df.FirstDate.On(df.SecondDate)), "Equal")
		assertSet("notOn", matchNames(df.FirstDate.NotOn(df.SecondDate)), "FirstBefore", "FirstAfter", "Between")
		assertSet("before", matchNames(df.FirstDate.Before(df.SecondDate)), "FirstBefore")
		assertSet("after", matchNames(df.FirstDate.After(df.SecondDate)), "FirstAfter", "Between")
		assertSet("onOrBefore", matchNames(df.FirstDate.OnOrBefore(df.SecondDate)), "Equal", "FirstBefore")
		assertSet("onOrAfter", matchNames(df.FirstDate.OnOrAfter(df.SecondDate)), "Equal", "FirstAfter", "Between")
		assertSet("between", matchNames(df.FirstDate.Between(df.SecondDate, df.ThirdDate, true)), "Equal", "Between")
		assertSet("betweenExcl", matchNames(df.FirstDate.Between(df.SecondDate, df.ThirdDate, false)), "Between")
		assertSet("betweenStrLit", matchNames(df.FirstDate.Between(airtable.DateLiteral("2024-01-01"), df.SecondDate, true)), "Equal", "FirstBefore")
	})
}

// ---- small helpers ----------------------------------------------------------

func containsID(rows []*airtable.PrimaryModel, id string) bool {
	for _, r := range rows {
		if r.ID() == id {
			return true
		}
	}
	return false
}

func dictIDs(recs []*airtable.DictRecord) []string {
	out := make([]string, 0, len(recs))
	for _, r := range recs {
		out = append(out, r.ID)
	}
	return out
}

func equalStrings(a, b []string) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func itoa(n int) string { return strconv.Itoa(n) }
