using System.Text.Json.Nodes;
using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// TC11 — Multi-field sort + sort combined with a filter. The base filter suite only covers a
/// single-field sort. This verifies a two-field sort (primary key with ties broken by a secondary
/// key) and sorting within a filtered scope. Parity target for the other 8 suites.
/// </summary>
public class TestMultiFieldSort
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();

    private static Fields Row(string suite, long number, string text)
    {
        var fields = new Fields(new Dictionary<string, JsonNode?>(), PrimaryFields.NameToId);
        fields.SetString(PrimaryFields.PrimaryKeyId, $"{suite} {text}");
        fields.SetLong(PrimaryFields.NumberIntId, number);
        fields.SetString(PrimaryFields.SingleLineTextId, text);
        return fields;
    }

    private static string ScopeTo(IEnumerable<string> ids) =>
        Formulas.Or(ids.Select(id => $"RECORD_ID()='{id}'"));

    private List<string> Texts(IEnumerable<DictTable.Record> records) =>
        records.Select(r => r.Fields.GetString(PrimaryFields.SingleLineTextId)!).ToList();

    [Fact]
    public async Task TwoFieldSortBreaksTiesOnSecondKey()
    {
        var suite = TestSetup.PrimaryKey("Sort", "TwoField");
        // NumberInt ties at 10 (rows "b" and "a"); the secondary SingleLineText sort orders them.
        var created = await _airtable.Primary.Dict.CreateAsync(
            new List<Fields> { Row(suite, 10, "b"), Row(suite, 10, "a"), Row(suite, 20, "c") }
        );
        var ids = created.Select(r => r.Id).ToList();
        try
        {
            var results = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery()
                    .WithFormula(ScopeTo(ids))
                    .WithSort(PrimaryFields.NumberIntId, SortDirection.Asc)
                    .WithSort(PrimaryFields.SingleLineTextId, SortDirection.Asc)
            );
            // (10,a), (10,b), (20,c) — tie on 10 broken by text asc.
            Assert.Equal(new List<string> { "a", "b", "c" }, Texts(results));
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task SecondaryDescendingReversesTiedGroup()
    {
        var suite = TestSetup.PrimaryKey("Sort", "MixedDir");
        var created = await _airtable.Primary.Dict.CreateAsync(
            new List<Fields> { Row(suite, 10, "a"), Row(suite, 10, "b"), Row(suite, 20, "c") }
        );
        var ids = created.Select(r => r.Id).ToList();
        try
        {
            var results = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery()
                    .WithFormula(ScopeTo(ids))
                    .WithSort(PrimaryFields.NumberIntId, SortDirection.Asc)
                    .WithSort(PrimaryFields.SingleLineTextId, SortDirection.Desc)
            );
            // NumberInt asc, then text DESC within the 10-tie: (10,b), (10,a), (20,c).
            Assert.Equal(new List<string> { "b", "a", "c" }, Texts(results));
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task SortCombinedWithAFilter()
    {
        var suite = TestSetup.PrimaryKey("Sort", "WithFilter");
        var created = await _airtable.Primary.Dict.CreateAsync(
            new List<Fields>
            {
                Row(suite, 30, "x"),
                Row(suite, 10, "y"),
                Row(suite, 20, "z"),
                Row(suite, 5, "low"), // filtered out by NumberInt > 5
            }
        );
        var ids = created.Select(r => r.Id).ToList();
        try
        {
            var filter = Formulas.And(ScopeTo(ids), PrimaryModel.F.NumberInt.GreaterThan(5));
            var results = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery()
                    .WithFormula(filter)
                    .WithSort(PrimaryFields.NumberIntId, SortDirection.Asc)
            );
            // Filtered to NumberInt > 5, sorted asc: 10(y), 20(z), 30(x).
            Assert.Equal(new List<string> { "y", "z", "x" }, Texts(results));
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
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
}
