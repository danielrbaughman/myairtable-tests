using System.Text.Json.Nodes;
using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>Dict-style (untyped) CRUD against the live base via <c>airtable.Primary.Dict</c>.</summary>
public class TestStructCrudViaTable
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();

    [Fact]
    public async Task PrimaryKeyOnlyCrudRoundTrip()
    {
        var primaryKey = TestSetup.PrimaryKey("StructCrud", "PKOnly");
        var fields = new Fields(new Dictionary<string, JsonNode?>(), PrimaryFields.NameToId);
        fields.SetString(PrimaryFields.PrimaryKeyId, primaryKey);

        var created = await _airtable.Primary.Dict.CreateAsync(fields);
        var recordId = created.Id;
        try
        {
            Assert.False(string.IsNullOrEmpty(created.Id));
            Assert.Equal(primaryKey, created.Fields.GetString(PrimaryFields.PrimaryKeyId));

            var fetched = await _airtable.Primary.Dict.GetAsync(recordId);
            Assert.Equal(recordId, fetched.Id);
            Assert.Equal(primaryKey, fetched.Fields.GetString(PrimaryFields.PrimaryKeyId));

            var update = new Fields(new Dictionary<string, JsonNode?>(), PrimaryFields.NameToId);
            var updatedKey = primaryKey + " Updated";
            update.SetString(PrimaryFields.PrimaryKeyId, updatedKey);
            var updated = await _airtable.Primary.Dict.UpdateAsync(recordId, update);
            Assert.Equal(updatedKey, updated.Fields.GetString(PrimaryFields.PrimaryKeyId));

            await _airtable.Primary.Dict.DeleteAsync(recordId);
            await Assert.ThrowsAnyAsync<AirtableException>(() =>
                _airtable.Primary.Dict.GetAsync(recordId)
            );
        }
        catch
        {
            await TryDeleteAsync(recordId);
            throw;
        }
    }

    [Fact]
    public async Task DictTableSupportsDualIdNameFieldAccess()
    {
        var primaryKey = TestSetup.PrimaryKey("StructCrud", "DualAccess");
        var fields = new Fields(new Dictionary<string, JsonNode?>(), PrimaryFields.NameToId);
        fields.SetString(PrimaryFields.PrimaryKeyId, primaryKey);

        var created = await _airtable.Primary.Dict.CreateAsync(fields);
        try
        {
            Assert.Equal(primaryKey, created.Fields.GetString(PrimaryFields.PrimaryKeyName));
            Assert.Equal(primaryKey, created.Fields.GetString(PrimaryFields.PrimaryKeyId));
        }
        finally
        {
            await TryDeleteAsync(created.Id);
        }
    }

    [Fact]
    public async Task AllSimplePropertiesRoundTrip()
    {
        var primaryKey = TestSetup.PrimaryKey("StructCrud", "AllProps");
        var fields = new Fields(new Dictionary<string, JsonNode?>(), PrimaryFields.NameToId);
        fields.SetString(PrimaryFields.PrimaryKeyId, primaryKey);
        fields.SetString(PrimaryFields.SingleLineTextId, "Hello World");
        fields.SetString(PrimaryFields.LongTextId, "Long text content");

        var created = await _airtable.Primary.Dict.CreateAsync(fields);
        try
        {
            var fetched = await _airtable.Primary.Dict.GetAsync(created.Id);
            Assert.Equal("Hello World", fetched.Fields.GetString(PrimaryFields.SingleLineTextId));
            Assert.Equal("Long text content", fetched.Fields.GetString(PrimaryFields.LongTextId));
        }
        finally
        {
            await TryDeleteAsync(created.Id);
        }
    }

    private async Task TryDeleteAsync(string recordId)
    {
        try
        {
            await _airtable.Primary.Dict.DeleteAsync(recordId);
        }
        catch (AirtableException)
        {
            // best-effort cleanup
        }
    }
}
