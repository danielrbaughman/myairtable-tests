using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// TC10 — Upsert depth. The base CRUD suite only covers single-record, single-merge-field upsert.
/// This adds a multi-field merge key (match must agree on ALL merge fields) and the multiple-match
/// error path (a merge key matching more than one record is rejected by Airtable). Note: the
/// generated client exposes only single-record upsert, so batch upsert isn't covered here.
/// Parity target for the other 8 suites.
/// </summary>
public class TestUpsertDepth
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();

    [Fact]
    public async Task UpsertMatchesOnMultipleMergeFields()
    {
        var suite = TestSetup.PrimaryKey("Upsert", "MultiKey");
        var ids = new List<string>();
        try
        {
            // Seed a record identified by the (PrimaryKey, SingleLineText) pair.
            var seed = await _airtable.Primary.CreateAsync(
                new PrimaryModel { PrimaryKey = suite, SingleLineText = "anchor" }
            );
            ids.Add(seed.Id!);

            var mergeOn = new[] { PrimaryFields.PrimaryKeyId, PrimaryFields.SingleLineTextId };

            // Same pair -> UPDATE the seed (matched on both fields).
            var update = await _airtable.Primary.UpsertAsync(
                new PrimaryModel
                {
                    PrimaryKey = suite,
                    SingleLineText = "anchor",
                    LongText = "updated",
                },
                mergeOn
            );
            Assert.False(update.WasCreated);
            Assert.Equal(seed.Id, update.Model.Id);
            Assert.Equal("updated", update.Model.LongText);

            // Same PrimaryKey but a DIFFERENT SingleLineText -> no match on the pair -> INSERT.
            var insert = await _airtable.Primary.UpsertAsync(
                new PrimaryModel { PrimaryKey = suite, SingleLineText = "different" },
                mergeOn
            );
            ids.Add(insert.Model.Id!);
            Assert.True(insert.WasCreated);
            Assert.NotEqual(seed.Id, insert.Model.Id);
        }
        finally
        {
            await TryDeleteManyAsync(ids);
        }
    }

    [Fact]
    public async Task UpsertWithMultipleMatchesThrows()
    {
        var suite = TestSetup.PrimaryKey("Upsert", "MultiMatch");
        var ids = new List<string>();
        try
        {
            // Two records share the same SingleLineText value.
            var a = await _airtable.Primary.CreateAsync(
                new PrimaryModel { PrimaryKey = suite + " A", SingleLineText = "dupe" }
            );
            var b = await _airtable.Primary.CreateAsync(
                new PrimaryModel { PrimaryKey = suite + " B", SingleLineText = "dupe" }
            );
            ids.Add(a.Id!);
            ids.Add(b.Id!);

            // Upsert merging only on SingleLineText="dupe" matches BOTH -> Airtable rejects it.
            var ex = await Assert.ThrowsAnyAsync<AirtableException>(() =>
                _airtable.Primary.UpsertAsync(
                    new PrimaryModel { SingleLineText = "dupe", LongText = "x" },
                    new[] { PrimaryFields.SingleLineTextId }
                )
            );
            Assert.IsType<AirtableException.ApiError>(ex);
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
            await _airtable.Primary.DeleteAsync(ids);
        }
        catch (AirtableException) { }
    }
}
