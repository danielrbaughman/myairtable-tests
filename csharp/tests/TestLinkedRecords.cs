using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// CS6.2 — Linked records as raw record-ID lists (no wrapper type). Resolution happens by passing
/// the IDs to the linked table's <c>GetAsync</c>. Parity with the Java/Kotlin/Swift/Rust suites.
/// </summary>
public class TestLinkedRecords
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();

    [Fact]
    public async Task LinkedRecordFieldsRoundTripViaRecordIds()
    {
        var suite = TestSetup.PrimaryKey("Linked", "RoundTrip");
        string? sec1Id = null;
        string? sec2Id = null;
        string? primId = null;
        try
        {
            var sec1 = await _airtable.Secondary.CreateAsync(
                new SecondaryModel { Name = suite + " S1" }
            );
            sec1Id = sec1.Id;
            var sec2 = await _airtable.Secondary.CreateAsync(
                new SecondaryModel { Name = suite + " S2" }
            );
            sec2Id = sec2.Id;

            var prim = await _airtable.Primary.CreateAsync(
                new PrimaryModel
                {
                    PrimaryKey = suite,
                    LinkSingle = new List<string> { sec1Id! },
                    LinkMultiple = new List<string> { sec1Id!, sec2Id! },
                }
            );
            primId = prim.Id;
            Assert.Equal(new List<string> { sec1Id! }, prim.LinkSingle);
            Assert.Equal(new List<string> { sec1Id!, sec2Id! }, prim.LinkMultiple);

            var fetched = await _airtable.Primary.GetAsync(primId!);
            Assert.Equal(new List<string> { sec1Id! }, fetched.LinkSingle);
            Assert.Equal(new List<string> { sec1Id!, sec2Id! }, fetched.LinkMultiple);

            fetched.LinkSingle = new List<string> { sec2Id! };
            fetched.LinkMultiple = new List<string> { sec1Id! };
            var updated = await _airtable.Primary.UpdateAsync(fetched);
            Assert.Equal(new List<string> { sec2Id! }, updated.LinkSingle);
            Assert.Equal(new List<string> { sec1Id! }, updated.LinkMultiple);
        }
        finally
        {
            await TryDeletePrimaryAsync(primId);
            await TryDeleteSecondariesAsync(sec1Id, sec2Id);
        }
    }

    [Fact]
    public async Task LinkedSingleRecordResolvesViaSecondaryGet()
    {
        var suite = TestSetup.PrimaryKey("Linked", "Single");
        string? secId = null;
        string? primId = null;
        try
        {
            var sec = await _airtable.Secondary.CreateAsync(
                new SecondaryModel { Name = suite + " Target", Value = "sv" }
            );
            secId = sec.Id;

            var prim = await _airtable.Primary.CreateAsync(
                new PrimaryModel
                {
                    PrimaryKey = suite,
                    LinkSingle = new List<string> { secId! },
                }
            );
            primId = prim.Id;
            Assert.Single(prim.LinkSingle!);
            var linkedId = prim.LinkSingle![0];
            Assert.Equal(secId, linkedId);

            var linked = await _airtable.Secondary.GetAsync(linkedId);
            Assert.Equal(suite + " Target", linked.Name);
            Assert.Equal("sv", linked.Value);
        }
        finally
        {
            await TryDeletePrimaryAsync(primId);
            await TryDeleteSecondariesAsync(secId);
        }
    }

    [Fact]
    public async Task EmptyLinkSingleDecodesAsNull()
    {
        var suite = TestSetup.PrimaryKey("Linked", "FetchNil");
        var prim = await _airtable.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = suite + " No Links" }
        );
        var primId = prim.Id;
        try
        {
            // Airtable omits empty link fields entirely; linkSingle decodes as null/empty.
            Assert.True(
                prim.LinkSingle is null or { Count: 0 },
                "empty link field decodes as null/empty"
            );
        }
        finally
        {
            await TryDeletePrimaryAsync(primId);
        }
    }

    [Fact]
    public async Task LinkedMultiRecordsResolveViaSecondaryGet()
    {
        var suite = TestSetup.PrimaryKey("Linked", "Multi");
        string? sec1Id = null;
        string? sec2Id = null;
        string? primId = null;
        try
        {
            var sec1 = await _airtable.Secondary.CreateAsync(
                new SecondaryModel { Name = suite + " T1" }
            );
            sec1Id = sec1.Id;
            var sec2 = await _airtable.Secondary.CreateAsync(
                new SecondaryModel { Name = suite + " T2" }
            );
            sec2Id = sec2.Id;

            var prim = await _airtable.Primary.CreateAsync(
                new PrimaryModel
                {
                    PrimaryKey = suite,
                    LinkMultiple = new List<string> { sec1Id!, sec2Id! },
                }
            );
            primId = prim.Id;

            var linked = await _airtable.Secondary.GetAsync(
                prim.LinkMultiple ?? new List<string>()
            );
            Assert.Equal(2, linked.Count);
            var names = linked.Select(m => m.Name).ToList();
            Assert.Contains(suite + " T1", names);
            Assert.Contains(suite + " T2", names);
        }
        finally
        {
            await TryDeletePrimaryAsync(primId);
            await TryDeleteSecondariesAsync(sec1Id, sec2Id);
        }
    }

    // ---- best-effort cleanup helpers (skip null ids; swallow cleanup errors) ----

    private async Task TryDeletePrimaryAsync(string? primId)
    {
        if (string.IsNullOrEmpty(primId))
            return;
        try
        {
            await _airtable.Primary.DeleteAsync(primId);
        }
        catch (AirtableException)
        {
            // best-effort cleanup — surface the original failure
        }
    }

    private async Task TryDeleteSecondariesAsync(params string?[] secIds)
    {
        var ids = secIds.Where(id => !string.IsNullOrEmpty(id)).Select(id => id!).ToList();
        if (ids.Count == 0)
            return;
        try
        {
            await _airtable.Secondary.DeleteAsync(ids);
        }
        catch (AirtableException)
        {
            // best-effort cleanup — surface the original failure
        }
    }
}
