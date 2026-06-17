using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// CS9.5 — TTL cache behavior end-to-end: hits, default-off, mutation invalidation, manual
/// invalidation, TTL expiry, query-key separation. Parity with the Java/Kotlin/Swift/Rust suites.
/// </summary>
public class TestCaching
{
    private static Airtable Cached() => TestSetup.MakeAirtable(60.0);

    private static Airtable Uncached() => TestSetup.MakeAirtable(); // cacheSeconds defaults to 0

    private static async Task BestEffortDeleteAsync(Airtable at, string? id)
    {
        if (string.IsNullOrEmpty(id))
            return;
        try
        {
            await at.Primary.Dict.DeleteAsync(id);
        }
        catch (AirtableException) { }
    }

    [Fact]
    public async Task CacheHitReturnsSameRecordOnRepeatedGet()
    {
        var at = Cached();
        var created = await at.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = TestSetup.PrimaryKey("Cache", "Hit") }
        );
        var id = created.Id!;
        try
        {
            var first = await at.Primary.GetAsync(id);
            var second = await at.Primary.GetAsync(id);
            Assert.Equal(first.PrimaryKey, second.PrimaryKey);
            Assert.Equal(first.Id, second.Id);
            Assert.True(at.Client.Cache.Count > 0, "read populated the cache");
        }
        finally
        {
            await BestEffortDeleteAsync(at, id);
        }
    }

    [Fact]
    public async Task CacheIsDisabledWhenCacheSecondsIsZero()
    {
        var at = Uncached();
        Assert.Equal(0, at.Client.Cache.Count);
        var created = await at.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = TestSetup.PrimaryKey("Cache", "Off") }
        );
        var id = created.Id!;
        try
        {
            await at.Primary.GetAsync(id);
            Assert.Equal(0, at.Client.Cache.Count); // TTL=0 is a no-op
        }
        finally
        {
            await BestEffortDeleteAsync(at, id);
        }
    }

    [Fact]
    public async Task CreateInvalidatesCachedReadsForTheTable()
    {
        var at = Cached();
        var key = TestSetup.PrimaryKey("Cache", "MutateCreate");
        var a = await at.Primary.CreateAsync(new PrimaryModel { PrimaryKey = key });
        var idA = a.Id!;
        string? idB = null;
        try
        {
            await at.Primary.GetAsync(idA); // populate cache
            Assert.True(at.Client.Cache.Count > 0);

            var b = await at.Primary.CreateAsync(new PrimaryModel { PrimaryKey = key + " B" });
            idB = b.Id;
            Assert.Equal(0, at.Client.Cache.Count); // create wipes this table's cache
        }
        finally
        {
            await BestEffortDeleteAsync(at, idA);
            await BestEffortDeleteAsync(at, idB);
        }
    }

    [Fact]
    public async Task UpdateInvalidatesCachedReadsForTheTable()
    {
        var at = Cached();
        var created = await at.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = TestSetup.PrimaryKey("Cache", "MutateUpdate") }
        );
        var id = created.Id!;
        try
        {
            await at.Primary.GetAsync(id);
            Assert.True(at.Client.Cache.Count > 0);

            created.SingleLineText = "mutated";
            await at.Primary.UpdateAsync(created);
            Assert.Equal(0, at.Client.Cache.Count); // update wipes this table's cache
        }
        finally
        {
            await BestEffortDeleteAsync(at, id);
        }
    }

    [Fact]
    public async Task DeleteInvalidatesCachedReadsForTheTable()
    {
        var at = Cached();
        var created = await at.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = TestSetup.PrimaryKey("Cache", "MutateDelete") }
        );
        var id = created.Id!;
        var deleted = false;
        try
        {
            await at.Primary.GetAsync(id);
            Assert.True(at.Client.Cache.Count > 0);

            await at.Primary.DeleteAsync(id);
            deleted = true;
            Assert.Equal(0, at.Client.Cache.Count); // delete wipes this table's cache
        }
        finally
        {
            if (!deleted)
                await BestEffortDeleteAsync(at, id);
        }
    }

    [Fact]
    public async Task InvalidateCacheDropsThisTablesCache()
    {
        var at = Cached();
        var created = await at.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = TestSetup.PrimaryKey("Cache", "ManualTable") }
        );
        var id = created.Id!;
        try
        {
            await at.Primary.GetAsync(id);
            Assert.True(at.Client.Cache.Count > 0);

            at.Client.InvalidateCache(PrimaryTable.TableId);
            Assert.Equal(0, at.Client.Cache.Count);
        }
        finally
        {
            await BestEffortDeleteAsync(at, id);
        }
    }

    [Fact]
    public async Task InvalidateAllCachesDropsEveryTablesCache()
    {
        var at = Cached();
        var created = await at.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = TestSetup.PrimaryKey("Cache", "ManualAll") }
        );
        var id = created.Id!;
        try
        {
            await at.Primary.GetAsync(id);
            await at.Secondary.GetAsync(new AirtableQuery().WithMaxRecords(1));
            Assert.True(at.Client.Cache.Count >= 2);

            at.InvalidateAllCaches();
            Assert.Equal(0, at.Client.Cache.Count);
        }
        finally
        {
            await BestEffortDeleteAsync(at, id);
        }
    }

    [Fact]
    public async Task CachedEntriesExpireAfterTheConfiguredTtl()
    {
        var at = TestSetup.MakeAirtable(1.0);
        var key = TestSetup.PrimaryKey("Cache", "TTL");
        var created = await at.Primary.CreateAsync(new PrimaryModel { PrimaryKey = key });
        var id = created.Id!;
        try
        {
            await at.Primary.GetAsync(id);
            Assert.True(at.Client.Cache.Count > 0);

            await Task.Delay(1500);
            // Probe the single-record entry directly: Cache.Get lazily evicts past TTL and returns
            // null, proving the entry expired rather than being served stale. Key mirrors
            // AirtableClient.GetRecordAsync: "rec:" + id.
            Assert.Null(at.Client.Cache.Get(PrimaryTable.TableId, "rec:" + id));

            var fresh = await at.Primary.GetAsync(id);
            Assert.Equal(key, fresh.PrimaryKey);
        }
        finally
        {
            await BestEffortDeleteAsync(at, id);
        }
    }

    [Fact]
    public async Task DifferentQueryParamsProduceDifferentCacheKeys()
    {
        var at = Cached();
        var created = await at.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = TestSetup.PrimaryKey("Cache", "Keys") }
        );
        var id = created.Id!;
        try
        {
            await at.Primary.GetAsync(new AirtableQuery().WithMaxRecords(1));
            await at.Primary.GetAsync(new AirtableQuery().WithMaxRecords(2));
            Assert.True(at.Client.Cache.Count >= 2, "distinct queries cache separately");
        }
        finally
        {
            await BestEffortDeleteAsync(at, id);
        }
    }
}
