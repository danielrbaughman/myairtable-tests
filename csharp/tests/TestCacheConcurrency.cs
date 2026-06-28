using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// TC9 — Cache thread-safety. The TTL cache is shared mutable state on the client; the caching
/// suite only exercises it single-threaded. These fire concurrent reads and concurrent mutations at
/// one cached client and assert no corruption/exception and a consistent result. (Run the Go port
/// under <c>-race</c> to surface unsynchronized map access.) Parity target for the other 8 suites.
/// </summary>
public class TestCacheConcurrency
{
    private static Airtable Cached() => TestSetup.MakeAirtable(60.0);

    [Fact]
    public async Task ConcurrentGetsOfTheSameRecordAreConsistent()
    {
        var at = Cached();
        var key = TestSetup.PrimaryKey("CacheConc", "Reads");
        var created = await at.Primary.CreateAsync(new PrimaryModel { PrimaryKey = key });
        var id = created.Id!;
        try
        {
            // 25 concurrent gets race to populate/read the shared cache.
            var results = await Task.WhenAll(
                Enumerable.Range(0, 25).Select(_ => at.Primary.GetAsync(id))
            );
            Assert.All(results, m => Assert.Equal(key, m.PrimaryKey));
            Assert.All(results, m => Assert.Equal(id, m.Id));
        }
        finally
        {
            await BestEffortDeleteAsync(at, id);
        }
    }

    [Fact]
    public async Task ConcurrentReadsAndMutationsDoNotCorruptTheCache()
    {
        var at = Cached();
        var key = TestSetup.PrimaryKey("CacheConc", "Mix");
        var created = await at.Primary.CreateAsync(new PrimaryModel { PrimaryKey = key });
        var id = created.Id!;
        try
        {
            // Interleave reads (populate cache), manual invalidations, and re-reads concurrently.
            var tasks = new List<Task>();
            for (var i = 0; i < 15; i++)
            {
                tasks.Add(at.Primary.GetAsync(id));
                tasks.Add(Task.Run(() => at.Client.InvalidateCache(PrimaryTable.TableId)));
                tasks.Add(Task.Run(() => at.InvalidateAllCaches()));
            }
            await Task.WhenAll(tasks);

            // The client is still usable and returns the correct value afterward.
            var fetched = await at.Primary.GetAsync(id);
            Assert.Equal(key, fetched.PrimaryKey);
        }
        finally
        {
            await BestEffortDeleteAsync(at, id);
        }
    }

    private static async Task BestEffortDeleteAsync(Airtable at, string? id)
    {
        if (string.IsNullOrEmpty(id))
            return;
        try
        {
            await at.Primary.DeleteAsync(id);
        }
        catch (AirtableException) { }
    }
}
