using System.Text.Json.Nodes;
using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// CS7.5 — Filter-by-formula integration tests. Parity with the Java/Kotlin/Swift/Rust
/// TestFilterByFormula suites: the generated <c>{Model}.F</c> filter DSL against the live base.
/// </summary>
public class TestFilterByFormula
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();

    // ---- shared helpers ----

    /// <summary>Scope a formula to specific record IDs so suite runs don't interfere.</summary>
    private static string ScopeTo(IReadOnlyList<string> ids)
    {
        var parts = ids.Select(id => $"RECORD_ID()='{id}'").ToList();
        return parts.Count == 1 ? parts[0] : Formulas.Or(parts);
    }

    private static Fields PrimaryFieldsFor(string name)
    {
        var fields = new Fields(new Dictionary<string, JsonNode?>(), PrimaryFields.NameToId);
        fields.SetString(PrimaryFields.PrimaryKeyId, name);
        return fields;
    }

    private static List<string> Names(IEnumerable<DictTable.Record> records) =>
        records
            .Select(r => r.Fields.GetString(PrimaryFields.PrimaryKeyId))
            .Where(n => n is not null)
            .Select(n => n!)
            .OrderBy(n => n, StringComparer.Ordinal)
            .ToList();

    /// <summary>Count of Primary dict records matching <c>formula</c> within <c>scope</c>.</summary>
    private async Task<int> CountAsync(string scope, string formula) =>
        (
            await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery().WithFormula(Formulas.And(scope, formula))
            )
        ).Count;

    // ---- text field filter ----

    [Fact]
    public async Task TextFieldEquals()
    {
        var suite = TestSetup.PrimaryKey("Filter", "TextEq");
        var created = await _airtable.Primary.Orm.CreateAsync(
            new PrimaryModel { PrimaryKey = suite, SingleLineText = "UniqueFilterValue" }
        );
        var recordId = created.Id!;
        try
        {
            var filter = PrimaryModel.F.SingleLineText.Eq("UniqueFilterValue");
            var results = await _airtable.Primary.Orm.GetAsync(
                new AirtableQuery().WithFormula(filter)
            );
            Assert.Contains(results, r => r.Id == recordId);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task RecordIdFilter()
    {
        var suite = TestSetup.PrimaryKey("Filter", "RecordID");
        var created = await _airtable.Primary.Orm.CreateAsync(
            new PrimaryModel { PrimaryKey = suite }
        );
        var recordId = created.Id!;
        try
        {
            var results = await _airtable.Primary.Orm.GetAsync(
                new AirtableQuery().WithFormula(PrimaryModel.F.Id.Eq(recordId))
            );
            Assert.Single(results);
            Assert.Equal(recordId, results[0].Id);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task NumberFieldFilter()
    {
        var suite = TestSetup.PrimaryKey("Filter", "Number");
        var c1 = await _airtable.Primary.Orm.CreateAsync(
            new PrimaryModel { NumberInt = 10.0, PrimaryKey = suite + " 1" }
        );
        var c2 = await _airtable.Primary.Orm.CreateAsync(
            new PrimaryModel { NumberInt = 20.0, PrimaryKey = suite + " 2" }
        );
        var ids = new List<string> { c1.Id!, c2.Id! };
        try
        {
            var results = await _airtable.Primary.Orm.GetAsync(
                new AirtableQuery().WithFormula(PrimaryModel.F.NumberInt.GreaterThan(15))
            );
            Assert.Contains(results, r => r.Id == c2.Id);
            Assert.DoesNotContain(results, r => r.Id == c1.Id);
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task AndCombinator()
    {
        var suite = TestSetup.PrimaryKey("Filter", "AND");
        var created = await _airtable.Primary.Orm.CreateAsync(
            new PrimaryModel
            {
                Checkbox = true,
                NumberInt = 42.0,
                PrimaryKey = suite,
            }
        );
        var recordId = created.Id!;
        try
        {
            var f = PrimaryModel.F;
            var filter = Formulas.And(f.Checkbox.IsTrue(), f.NumberInt.Eq(42));
            var results = await _airtable.Primary.Orm.GetAsync(
                new AirtableQuery().WithFormula(filter)
            );
            Assert.Contains(results, r => r.Id == recordId);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task FilterByView()
    {
        var creates = new List<Fields>();
        for (var i = 0; i < 5; i++)
            creates.Add(PrimaryFieldsFor("Filter Test " + i));
        for (var i = 0; i < 5; i++)
            creates.Add(PrimaryFieldsFor("Don't Include Test " + i));
        var created = await _airtable.Primary.Dict.CreateAsync(creates);
        var ids = created.Select(r => r.Id).ToList();
        try
        {
            var results = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery().WithView(PrimaryView.FilterByView)
            );
            Assert.True(results.Count >= 5);
            foreach (var record in results)
            {
                var name = record.Fields.GetString(PrimaryFields.PrimaryKeyId) ?? "";
                Assert.StartsWith("Filter Test", name);
            }
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task RecordIdInList()
    {
        var suite = TestSetup.PrimaryKey("Filter", "InList");
        var creates = Enumerable.Range(0, 3).Select(i => PrimaryFieldsFor($"{suite} {i}")).ToList();
        var created = await _airtable.Primary.Dict.CreateAsync(creates);
        var ids = created.Select(r => r.Id).ToList();
        try
        {
            var f = PrimaryModel.F;
            var multi = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery().WithFormula(f.Id.InList(new List<string> { ids[0], ids[1] }))
            );
            Assert.Equal(2, multi.Count);

            var single = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery().WithFormula(f.Id.InList(new List<string> { ids[0] }))
            );
            Assert.Single(single);

            var empty = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery().WithFormula(f.Id.InList(new List<string>()))
            );
            Assert.Empty(empty);
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task TextFieldPredicates()
    {
        var created = await _airtable.Primary.Dict.CreateAsync(
            new List<Fields>
            {
                PrimaryFieldsFor("FbText Alpha One"),
                PrimaryFieldsFor("FbText Alpha Two"),
                PrimaryFieldsFor("FbText Beta One"),
            }
        );
        var ids = created.Select(r => r.Id).ToList();
        var scope = ScopeTo(ids);
        var f = PrimaryModel.F;
        try
        {
            Assert.Equal(1, await CountAsync(scope, f.PrimaryKey.Eq("FbText Alpha One")));
            Assert.Equal(2, await CountAsync(scope, f.PrimaryKey.Neq("FbText Alpha One")));
            Assert.Equal(2, await CountAsync(scope, f.PrimaryKey.Contains("Alpha", false, true)));
            Assert.Equal(
                3,
                await CountAsync(
                    scope,
                    f.PrimaryKey.ContainsAny(new List<string> { "Alpha", "Beta" }, false, true)
                )
            );
            Assert.Equal(
                1,
                await CountAsync(
                    scope,
                    f.PrimaryKey.ContainsAll(new List<string> { "Alpha", "One" }, false, true)
                )
            );
            Assert.Equal(
                1,
                await CountAsync(scope, f.PrimaryKey.NotContains("Alpha", false, true))
            );
            Assert.Equal(
                2,
                await CountAsync(scope, f.PrimaryKey.StartsWith("FbText Alpha", false, true))
            );
            Assert.Equal(
                1,
                await CountAsync(scope, f.PrimaryKey.NotStartsWith("FbText Alpha", false, true))
            );
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task NumberFieldPredicates()
    {
        var suite = TestSetup.PrimaryKey("Filter", "NumPreds");
        var values = new long[] { 10, 20, 30 };
        var creates = new List<Fields>();
        for (var i = 0; i < values.Length; i++)
        {
            var fields = PrimaryFieldsFor($"{suite} {i}");
            fields.SetLong(PrimaryFields.NumberIntId, values[i]);
            creates.Add(fields);
        }
        var created = await _airtable.Primary.Dict.CreateAsync(creates);
        var ids = created.Select(r => r.Id).ToList();
        var scope = ScopeTo(ids);
        var f = PrimaryModel.F;
        try
        {
            Assert.Equal(1, await CountAsync(scope, f.NumberInt.Eq(20)));
            Assert.Equal(2, await CountAsync(scope, f.NumberInt.Neq(20)));
            Assert.Equal(2, await CountAsync(scope, f.NumberInt.GreaterThan(10)));
            Assert.Equal(2, await CountAsync(scope, f.NumberInt.LessThan(30)));
            Assert.Equal(2, await CountAsync(scope, f.NumberInt.GreaterThanOrEquals(20)));
            Assert.Equal(2, await CountAsync(scope, f.NumberInt.LessThanOrEquals(20)));
            Assert.Equal(3, await CountAsync(scope, f.NumberInt.Between(10, 30)));
            Assert.Equal(1, await CountAsync(scope, f.NumberInt.Between(10, 30, false)));
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task BooleanFieldFilter()
    {
        var suite = TestSetup.PrimaryKey("Filter", "Bool");
        var f1 = PrimaryFieldsFor(suite + " A");
        f1.SetBoolean(PrimaryFields.CheckboxId, true);
        var f2 = PrimaryFieldsFor(suite + " B");
        f2.SetBoolean(PrimaryFields.CheckboxId, false);
        var created = await _airtable.Primary.Dict.CreateAsync(new List<Fields> { f1, f2 });
        var ids = created.Select(r => r.Id).ToList();
        var scope = ScopeTo(ids);
        var f = PrimaryModel.F;
        try
        {
            Assert.Equal(1, await CountAsync(scope, f.Checkbox.Eq(true)));
            Assert.True(await CountAsync(scope, f.Checkbox.Eq(false)) >= 1);
            Assert.Equal(1, await CountAsync(scope, f.Checkbox.IsTrue()));
            Assert.True(await CountAsync(scope, f.Checkbox.IsFalse()) >= 1);
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task AttachmentsFieldFilter()
    {
        var suite = TestSetup.PrimaryKey("Filter", "Attach");
        var f1 = PrimaryFieldsFor(suite + " A");
        var attachments = new JsonArray(
            new JsonObject
            {
                ["url"] =
                    "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png",
            }
        );
        f1.Set(PrimaryFields.AttachmentId, attachments);
        var f2 = PrimaryFieldsFor(suite + " B");
        var created = await _airtable.Primary.Dict.CreateAsync(new List<Fields> { f1, f2 });
        var ids = created.Select(r => r.Id).ToList();
        var scope = ScopeTo(ids);
        var f = PrimaryModel.F;
        try
        {
            var notEmpty = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery().WithFormula(Formulas.And(scope, f.Attachment.NotEmpty()))
            );
            Assert.NotEmpty(notEmpty);
            var empty = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery().WithFormula(Formulas.And(scope, f.Attachment.Empty()))
            );
            Assert.NotEmpty(empty);
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task DateFieldPredicates()
    {
        var suite = TestSetup.PrimaryKey("Filter", "Date");
        var f1 = PrimaryFieldsFor(suite + " A");
        f1.SetString(PrimaryFields.DateId, "2024-01-15");
        var f2 = PrimaryFieldsFor(suite + " B");
        f2.SetString(PrimaryFields.DateId, "2024-06-15");
        var f3 = PrimaryFieldsFor(suite + " C"); // no date
        var created = await _airtable.Primary.Dict.CreateAsync(new List<Fields> { f1, f2, f3 });
        var ids = created.Select(r => r.Id).ToList();
        var scope = ScopeTo(ids);
        var f = PrimaryModel.F;
        try
        {
            Assert.Equal(1, await CountAsync(scope, f.Date.On("2024-01-15")));
            Assert.True(await CountAsync(scope, f.Date.NotOn("2024-01-15")) >= 1);
            Assert.True(await CountAsync(scope, f.Date.OnOrAfter("2024-06-15")) >= 1);
            Assert.True(await CountAsync(scope, f.Date.OnOrBefore("2024-01-15")) >= 1);
            Assert.True(await CountAsync(scope, f.Date.After("2024-01-15")) >= 1);
            Assert.True(await CountAsync(scope, f.Date.Before("2024-06-15")) >= 1);
            Assert.Equal(2, await CountAsync(scope, f.Date.Between("2024-01-15", "2024-06-15")));
            Assert.Equal(
                2,
                await CountAsync(scope, f.Date.Between("2024-01-01", "2024-12-31", false))
            );
            Assert.True(await CountAsync(scope, f.Date.NotEmpty()) >= 1);
            Assert.True(await CountAsync(scope, f.Date.Empty()) >= 1);
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task DateFieldChained()
    {
        var suite = TestSetup.PrimaryKey("Filter", "DateChain");
        var f1 = PrimaryFieldsFor(suite + " A");
        f1.SetString(PrimaryFields.DateId, "2024-01-15");
        var f2 = PrimaryFieldsFor(suite + " B");
        f2.SetString(PrimaryFields.DateId, "2024-06-15");
        var created = await _airtable.Primary.Dict.CreateAsync(new List<Fields> { f1, f2 });
        var ids = created.Select(r => r.Id).ToList();
        var scope = ScopeTo(ids);
        var f = PrimaryModel.F;
        try
        {
            var past = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery().WithFormula(Formulas.And(scope, f.Date.Before().DaysAgo(1)))
            );
            Assert.NotEmpty(past);
            var recent = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery().WithFormula(Formulas.And(scope, f.Date.After().YearsAgo(100)))
            );
            Assert.NotEmpty(recent);
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task ComplexFormulas()
    {
        var suite = TestSetup.PrimaryKey("Filter", "Complex");
        var f1 = PrimaryFieldsFor(suite + " A");
        f1.SetLong(PrimaryFields.NumberIntId, 10);
        f1.SetBoolean(PrimaryFields.CheckboxId, true);
        var f2 = PrimaryFieldsFor(suite + " B");
        f2.SetLong(PrimaryFields.NumberIntId, 20);
        f2.SetBoolean(PrimaryFields.CheckboxId, false);
        var f3 = PrimaryFieldsFor(suite + " C");
        f3.SetLong(PrimaryFields.NumberIntId, 30);
        f3.SetBoolean(PrimaryFields.CheckboxId, true);
        var created = await _airtable.Primary.Dict.CreateAsync(new List<Fields> { f1, f2, f3 });
        var ids = created.Select(r => r.Id).ToList();
        var scope = ScopeTo(ids);
        var f = PrimaryModel.F;
        try
        {
            Assert.Equal(
                1,
                await CountAsync(
                    scope,
                    Formulas.And(f.NumberInt.GreaterThan(10), f.Checkbox.IsTrue())
                )
            );
            Assert.Equal(
                2,
                await CountAsync(
                    scope,
                    Formulas.And(
                        f.PrimaryKey.Contains(suite, false, true),
                        f.NumberInt.GreaterThanOrEquals(20)
                    )
                )
            );
            Assert.Equal(
                2,
                await CountAsync(scope, Formulas.Or(f.NumberInt.Eq(10), f.NumberInt.Eq(30)))
            );
            Assert.Equal(
                3,
                await CountAsync(scope, Formulas.Or(f.Checkbox.IsTrue(), f.NumberInt.Eq(20)))
            );
            Assert.Equal(
                1,
                await CountAsync(
                    scope,
                    Formulas.Xor(f.Checkbox.IsTrue(), f.NumberInt.GreaterThanOrEquals(30))
                )
            );
            Assert.Equal(2, await CountAsync(scope, Formulas.Not(f.PrimaryKey.Eq(suite + " A"))));
            Assert.Equal(
                2,
                await CountAsync(
                    scope,
                    Formulas.And(
                        Formulas.Or(f.NumberInt.Eq(10), f.NumberInt.Eq(30)),
                        f.Checkbox.IsTrue()
                    )
                )
            );
            Assert.Equal(
                2,
                await CountAsync(
                    scope,
                    Formulas.Or(
                        Formulas.And(f.NumberInt.GreaterThan(10), f.Checkbox.IsTrue()),
                        f.PrimaryKey.Eq(suite + " A")
                    )
                )
            );
            Assert.Equal(
                2,
                await CountAsync(
                    scope,
                    Formulas.Not(Formulas.And(f.Checkbox.IsTrue(), f.NumberInt.GreaterThan(20)))
                )
            );
            Assert.Equal(
                1,
                await CountAsync(
                    scope,
                    Formulas.And(
                        Formulas.Not(f.Checkbox.IsTrue()),
                        f.NumberInt.GreaterThanOrEquals(20)
                    )
                )
            );
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task MaxRecords()
    {
        var suite = TestSetup.PrimaryKey("Filter", "MaxRec");
        var creates = Enumerable.Range(1, 5).Select(i => PrimaryFieldsFor($"{suite} {i}")).ToList();
        var created = await _airtable.Primary.Dict.CreateAsync(creates);
        var ids = created.Select(r => r.Id).ToList();
        try
        {
            var results = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery().WithFormula(ScopeTo(ids)).WithMaxRecords(3)
            );
            Assert.Equal(3, results.Count);
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task SortRecords()
    {
        var suite = TestSetup.PrimaryKey("Filter", "Sort");
        var f1 = PrimaryFieldsFor(suite + " C");
        f1.SetLong(PrimaryFields.NumberIntId, 30);
        var f2 = PrimaryFieldsFor(suite + " A");
        f2.SetLong(PrimaryFields.NumberIntId, 10);
        var f3 = PrimaryFieldsFor(suite + " B");
        f3.SetLong(PrimaryFields.NumberIntId, 20);
        var created = await _airtable.Primary.Dict.CreateAsync(new List<Fields> { f1, f2, f3 });
        var ids = created.Select(r => r.Id).ToList();
        var scope = ScopeTo(ids);
        try
        {
            var asc = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery().WithFormula(scope).WithSort(PrimaryFields.NumberIntId)
            );
            Assert.Equal(
                new List<long> { 10, 20, 30 },
                asc.Select(r => r.Fields.GetLong(PrimaryFields.NumberIntId))
                    .Where(v => v is not null)
                    .Select(v => v!.Value)
                    .ToList()
            );

            var desc = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery()
                    .WithFormula(scope)
                    .WithSort(PrimaryFields.NumberIntId, SortDirection.Desc)
            );
            Assert.Equal(
                new List<long> { 30, 20, 10 },
                desc.Select(r => r.Fields.GetLong(PrimaryFields.NumberIntId))
                    .Where(v => v is not null)
                    .Select(v => v!.Value)
                    .ToList()
            );
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task FieldSelection()
    {
        var suite = TestSetup.PrimaryKey("Filter", "FieldSel");
        var fields = PrimaryFieldsFor(suite);
        fields.SetString(PrimaryFields.SingleLineTextId, "Hello");
        fields.SetLong(PrimaryFields.NumberIntId, 42);
        var created = await _airtable.Primary.Dict.CreateAsync(fields);
        try
        {
            var results = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery()
                    .WithFormula($"RECORD_ID()='{created.Id}'")
                    .WithFields(PrimaryFields.PrimaryKeyId)
            );
            Assert.Single(results);
            var f = results[0].Fields;
            Assert.NotNull(f.GetString(PrimaryFields.PrimaryKeyId));
            Assert.Null(f.GetString(PrimaryFields.SingleLineTextId));
            Assert.Null(f.GetLong(PrimaryFields.NumberIntId));
        }
        finally
        {
            await TryDeleteAsync(created.Id);
        }
    }

    [Fact]
    public async Task LookupFieldFilter()
    {
        var suite = TestSetup.PrimaryKey("Filter", "Lookup");
        var secA = await _airtable.Secondary.Orm.CreateAsync(
            new SecondaryModel { Name = suite + " Sec A", Value = "Groundwork BioAg" }
        );
        var secB = await _airtable.Secondary.Orm.CreateAsync(
            new SecondaryModel { Name = suite + " Sec B", Value = "Groundwork Lab" }
        );
        var secC = await _airtable.Secondary.Orm.CreateAsync(
            new SecondaryModel { Name = suite + " Sec C", Value = "Other Vendor" }
        );
        var secIds = new List<string> { secA.Id!, secB.Id!, secC.Id! };

        var primA = await _airtable.Primary.Orm.CreateAsync(
            new PrimaryModel
            {
                LinkSingle = new List<string> { secA.Id! },
                PrimaryKey = suite + " A",
            }
        );
        var primB = await _airtable.Primary.Orm.CreateAsync(
            new PrimaryModel
            {
                LinkSingle = new List<string> { secB.Id! },
                PrimaryKey = suite + " B",
            }
        );
        var primC = await _airtable.Primary.Orm.CreateAsync(
            new PrimaryModel
            {
                LinkSingle = new List<string> { secC.Id! },
                PrimaryKey = suite + " C",
            }
        );
        var primIds = new List<string> { primA.Id!, primB.Id!, primC.Id! };
        var scope = ScopeTo(primIds);
        var f = PrimaryModel.F;
        try
        {
            Assert.Equal(
                new List<string> { suite + " A", suite + " B" },
                await MatchNamesAsync(scope, f.Lookup.Contains("groundwork", false, true))
            );
            Assert.Empty(
                await MatchNamesAsync(
                    scope,
                    f.Lookup.Contains("nonexistent-substring-xyz", false, true)
                )
            );
            Assert.Equal(
                new List<string> { suite + " A", suite + " B" },
                await MatchNamesAsync(scope, f.Lookup.StartsWith("Ground", false, true))
            );
            Assert.Equal(
                new List<string> { suite + " A" },
                await MatchNamesAsync(scope, f.Lookup.EndsWith("BioAg", false, true))
            );
            Assert.Equal(
                new List<string> { suite + " A" },
                await MatchNamesAsync(scope, f.Lookup.Eq("Groundwork BioAg"))
            );
            Assert.Equal(
                new List<string> { suite + " C" },
                await MatchNamesAsync(scope, f.Lookup.NotContains("Groundwork", false, true))
            );
        }
        finally
        {
            await TryDeleteManyAsync(primIds);
            await TryDeleteSecondariesAsync(secIds);
        }
    }

    private async Task<List<string>> MatchNamesAsync(string scope, string formula)
    {
        var records = await _airtable.Primary.Dict.GetAsync(
            new AirtableQuery().WithFormula(Formulas.And(scope, formula))
        );
        return Names(records);
    }

    [Fact]
    public async Task DateFieldVsField()
    {
        var suite = TestSetup.PrimaryKey("Filter", "VsField");
        var f = FormulasModel.F;

        var created = await _airtable.Formulas.Orm.CreateAsync(
            new List<FormulasModel>
            {
                MkFormulas(
                    suite,
                    "Equal",
                    "2024-06-15",
                    "2024-06-15T00:00:00.000Z",
                    "2024-06-15T00:00:00.000Z"
                ),
                MkFormulas(
                    suite,
                    "FirstBefore",
                    "2024-01-15",
                    "2024-06-15T00:00:00.000Z",
                    "2024-12-15T00:00:00.000Z"
                ),
                MkFormulas(
                    suite,
                    "FirstAfter",
                    "2024-12-15",
                    "2024-06-15T00:00:00.000Z",
                    "2024-01-15T00:00:00.000Z"
                ),
                MkFormulas(
                    suite,
                    "Between",
                    "2024-06-15",
                    "2024-01-15T00:00:00.000Z",
                    "2024-12-15T00:00:00.000Z"
                ),
            }
        );
        var ids = created.Select(m => m.Id!).ToList();
        var scope = ScopeTo(ids);
        try
        {
            Assert.Equal(
                new HashSet<string> { "Equal" },
                await FormulasMatchNamesAsync(suite, scope, f.FirstDate.On(f.SecondDate))
            );
            Assert.Equal(
                new HashSet<string> { "FirstBefore", "FirstAfter", "Between" },
                await FormulasMatchNamesAsync(suite, scope, f.FirstDate.NotOn(f.SecondDate))
            );
            Assert.Equal(
                new HashSet<string> { "FirstBefore" },
                await FormulasMatchNamesAsync(suite, scope, f.FirstDate.Before(f.SecondDate))
            );
            Assert.Equal(
                new HashSet<string> { "FirstAfter", "Between" },
                await FormulasMatchNamesAsync(suite, scope, f.FirstDate.After(f.SecondDate))
            );
            Assert.Equal(
                new HashSet<string> { "Equal", "FirstBefore" },
                await FormulasMatchNamesAsync(suite, scope, f.FirstDate.OnOrBefore(f.SecondDate))
            );
            Assert.Equal(
                new HashSet<string> { "Equal", "FirstAfter", "Between" },
                await FormulasMatchNamesAsync(suite, scope, f.FirstDate.OnOrAfter(f.SecondDate))
            );
            Assert.Equal(
                new HashSet<string> { "Equal", "Between" },
                await FormulasMatchNamesAsync(
                    suite,
                    scope,
                    f.FirstDate.Between(f.SecondDate, f.ThirdDate)
                )
            );
            Assert.Equal(
                new HashSet<string> { "Between" },
                await FormulasMatchNamesAsync(
                    suite,
                    scope,
                    f.FirstDate.Between(f.SecondDate, f.ThirdDate, false)
                )
            );
            Assert.Equal(
                new HashSet<string> { "Equal", "FirstBefore" },
                await FormulasMatchNamesAsync(
                    suite,
                    scope,
                    f.FirstDate.Between("2024-01-01", f.SecondDate)
                )
            );
        }
        finally
        {
            await TryDeleteFormulasAsync(ids);
        }
    }

    private static DateTimeOffset Utc(string s) =>
        DateTimeOffset.Parse(
            s,
            System.Globalization.CultureInfo.InvariantCulture,
            System.Globalization.DateTimeStyles.AssumeUniversal
                | System.Globalization.DateTimeStyles.AdjustToUniversal
        );

    private static FormulasModel MkFormulas(
        string suite,
        string name,
        string first,
        string second,
        string third
    ) =>
        new()
        {
            // Bare date strings must be read as UTC midnight (parity with the other targets); a
            // local-time parse shifts "First Date" off midnight and the date-only column rejects it.
            FirstDate = Utc(first),
            PrimaryKey = suite + " " + name,
            SecondDate = Utc(second),
            ThirdDate = Utc(third),
        };

    private async Task<HashSet<string>> FormulasMatchNamesAsync(
        string suite,
        string scope,
        string formula
    )
    {
        var models = await _airtable.Formulas.Orm.GetAsync(
            new AirtableQuery().WithFormula(Formulas.And(scope, formula))
        );
        return models
            .Select(m => m.PrimaryKey)
            .Where(n => n is not null)
            .Select(n => n!.Replace(suite + " ", ""))
            .ToHashSet();
    }

    // ---- cleanup helpers ----

    private async Task TryDeleteAsync(string? recordId)
    {
        if (string.IsNullOrEmpty(recordId))
            return;
        try
        {
            await _airtable.Primary.Dict.DeleteAsync(recordId);
        }
        catch (AirtableException) { }
    }

    private async Task TryDeleteManyAsync(IReadOnlyList<string> ids)
    {
        if (ids.Count == 0)
            return;
        try
        {
            await _airtable.Primary.Dict.DeleteAsync(ids);
        }
        catch (AirtableException) { }
    }

    private async Task TryDeleteSecondariesAsync(IReadOnlyList<string> ids)
    {
        try
        {
            await _airtable.Secondary.Dict.DeleteAsync(ids);
        }
        catch (AirtableException) { }
    }

    private async Task TryDeleteFormulasAsync(IReadOnlyList<string> ids)
    {
        try
        {
            await _airtable.Formulas.Dict.DeleteAsync(ids);
        }
        catch (AirtableException) { }
    }
}
