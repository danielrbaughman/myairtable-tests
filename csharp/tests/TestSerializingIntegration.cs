using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// CS10.1 — Network-bound serialization cases (split from the offline <see cref="TestSerializing"/>):
/// id + createdTime round-trip after fetch, linked-record fields serializing as record-ID arrays,
/// and an explicit null clearing a writable field server-side. Parity with the Java/Kotlin suites.
/// </summary>
public class TestSerializingIntegration
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();

    [Fact]
    public async Task RoundTripPreservesIdAndCreatedTimeAfterFetch()
    {
        var primaryKey = TestSetup.PrimaryKey("Serializing", "IdTime");
        var created = await _airtable.Primary.Orm.CreateAsync(
            new PrimaryModel { PrimaryKey = primaryKey }
        );
        var recordId = created.Id!;
        try
        {
            Assert.NotNull(created.CreatedTime); // populated on create
            var fetched = await _airtable.Primary.Orm.GetAsync(recordId);
            Assert.Equal(recordId, fetched.Id);
            Assert.Equal(created.CreatedTime, fetched.CreatedTime);
        }
        finally
        {
            await TryDeletePrimaryAsync(recordId);
        }
    }

    [Fact]
    public async Task LinkedRecordFieldsSerializeAsRecordIdArrays()
    {
        var suite = TestSetup.PrimaryKey("Serializing", "Links");
        string? secId = null;
        string? primId = null;
        try
        {
            var sec = await _airtable.Secondary.Orm.CreateAsync(
                new SecondaryModel { Name = suite + " Target" }
            );
            secId = sec.Id;
            var prim = await _airtable.Primary.Orm.CreateAsync(
                new PrimaryModel
                {
                    PrimaryKey = suite,
                    LinkSingle = new List<string> { secId! },
                }
            );
            primId = prim.Id;

            var record = prim.ToRecord();
            var linkElement = record[PrimaryFields.LinkSingleId];
            Assert.NotNull(linkElement);
            var array = linkElement!.AsArray(); // links serialize as arrays
            Assert.Equal(secId, array[0]!.GetValue<string>());
        }
        finally
        {
            await TryDeletePrimaryAsync(primId);
            await TryDeleteSecondaryAsync(secId);
        }
    }

    [Fact]
    public async Task ExplicitNullOnAWritableFieldClearsItServerSide()
    {
        var primaryKey = TestSetup.PrimaryKey("Serializing", "Clear");
        var created = await _airtable.Primary.Orm.CreateAsync(
            new PrimaryModel { PrimaryKey = primaryKey, SingleLineText = "to be cleared" }
        );
        var recordId = created.Id!;
        try
        {
            created.SingleLineText = null;
            var saved = await _airtable.Primary.Orm.UpdateAsync(created);
            Assert.Null(saved.SingleLineText); // null dirty entry clears the field

            var fetched = await _airtable.Primary.Orm.GetAsync(recordId);
            Assert.Null(fetched.SingleLineText);
        }
        finally
        {
            await TryDeletePrimaryAsync(recordId);
        }
    }

    private async Task TryDeletePrimaryAsync(string? id)
    {
        if (string.IsNullOrEmpty(id))
            return;
        try
        {
            await _airtable.Primary.Orm.DeleteAsync(id);
        }
        catch (AirtableException) { }
    }

    private async Task TryDeleteSecondaryAsync(string? id)
    {
        if (string.IsNullOrEmpty(id))
            return;
        try
        {
            await _airtable.Secondary.Orm.DeleteAsync(id);
        }
        catch (AirtableException) { }
    }
}
