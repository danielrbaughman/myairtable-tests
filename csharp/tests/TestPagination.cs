using System.Text.Json.Nodes;
using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// TC1 — Multi-page pagination: list more records than Airtable's 100-record default page in a
/// single query (no maxRecords) and assert the client's offset loop reassembles every page.
/// Exercises the untested page-reassembly path in the generated client. Parity target for the
/// other 8 suites.
/// </summary>
public class TestPagination
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();

    // 105 > the 100-record default page size, so a no-maxRecords list spans two pages.
    private const int Count = 105;

    [Fact]
    public async Task OrmGetSpanningMultiplePagesReturnsEveryRecord()
    {
        var suite = TestSetup.PrimaryKey("Pagination", "Orm");
        var models = Enumerable
            .Range(1, Count)
            .Select(i => new PrimaryModel { PrimaryKey = $"{suite} {i}" })
            .ToList();
        var createdIds = new List<string>();
        try
        {
            var created = await _airtable.Primary.CreateAsync(models);
            createdIds = created.Select(m => m.Id!).ToList();
            Assert.Equal(Count, created.Count);

            // No maxRecords: the offset loop must walk both pages and return all 105.
            var results = await _airtable.Primary.GetAsync(
                new AirtableQuery().WithFormula($"FIND(\"{suite}\", {{Primary Key}})")
            );
            Assert.Equal(Count, results.Count);
            Assert.Equal(
                new HashSet<string>(createdIds),
                new HashSet<string>(results.Select(m => m.Id!))
            );
        }
        finally
        {
            await TryDeleteManyAsync(createdIds, suite);
        }
    }

    [Fact]
    public async Task DictGetSpanningMultiplePagesReturnsEveryRecord()
    {
        var suite = TestSetup.PrimaryKey("Pagination", "Dict");
        var creates = Enumerable
            .Range(1, Count)
            .Select(i =>
            {
                var fields = new Fields(
                    new Dictionary<string, JsonNode?>(),
                    PrimaryFields.NameToId
                );
                fields.SetString(PrimaryFields.PrimaryKeyId, $"{suite} {i}");
                return fields;
            })
            .ToList();
        var createdIds = new List<string>();
        try
        {
            var created = await _airtable.Primary.Dict.CreateAsync(creates);
            createdIds = created.Select(r => r.Id).ToList();
            Assert.Equal(Count, created.Count);

            var results = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery().WithFormula($"FIND(\"{suite}\", {{Primary Key}})")
            );
            Assert.Equal(Count, results.Count);
            Assert.Equal(
                new HashSet<string>(createdIds),
                new HashSet<string>(results.Select(r => r.Id))
            );
        }
        finally
        {
            await TryDeleteManyAsync(createdIds, suite);
        }
    }

    // ---- cleanup ----

    private async Task TryDeleteManyAsync(IReadOnlyList<string> ids, string suite)
    {
        if (ids.Count > 0)
        {
            try
            {
                await _airtable.Primary.Dict.DeleteAsync(ids);
                return;
            }
            catch (AirtableException)
            {
                // fall through to a prefix sweep
            }
        }
        try
        {
            var stray = await _airtable.Primary.Dict.GetAsync(
                new AirtableQuery().WithFormula($"FIND(\"{suite}\", {{Primary Key}})")
            );
            if (stray.Count > 0)
                await _airtable.Primary.Dict.DeleteAsync(stray.Select(r => r.Id).ToList());
        }
        catch (AirtableException)
        {
            // best-effort cleanup
        }
    }
}
