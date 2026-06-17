using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// CS4.7 — Model-side CRUD (fluent <c>SaveAsync</c>/<c>FetchAsync</c>/<c>DeleteAsync</c> instance
/// methods). Parity with the Java/Kotlin/Swift/Rust TestOrmCrudViaModel suites.
/// </summary>
public class TestOrmCrudViaModel
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();

    [Fact]
    public async Task PrimaryKeyOnlyCrudViaModelMethods()
    {
        var primaryKey = TestSetup.PrimaryKey("OrmModel", "PKOnly");
        var created = await _airtable.Primary.Orm.CreateAsync(
            new PrimaryModel { PrimaryKey = primaryKey }
        );
        var recordId = created.Id!;
        try
        {
            // Mutate + SaveAsync() — no table argument needed.
            created.PrimaryKey = primaryKey + " Updated";
            var saved = await created.SaveAsync();
            Assert.Equal(primaryKey + " Updated", saved.PrimaryKey);
            Assert.Equal(recordId, saved.Id);

            // FetchAsync() returns a fresh server copy.
            var fetched = await saved.FetchAsync();
            Assert.Equal(primaryKey + " Updated", fetched.PrimaryKey);

            // DeleteAsync() via the model.
            await fetched.DeleteAsync();
            await Assert.ThrowsAnyAsync<AirtableException>(() =>
                _airtable.Primary.Orm.GetAsync(recordId)
            );
        }
        catch
        {
            await TryDeleteAsync(recordId);
            throw;
        }
    }

    [Fact]
    public async Task DirtyTrackingResetsAfterASuccessfulSave()
    {
        var primaryKey = TestSetup.PrimaryKey("OrmModel", "Dirty");
        var created = await _airtable.Primary.Orm.CreateAsync(
            new PrimaryModel { PrimaryKey = primaryKey }
        );
        var recordId = created.Id!;
        try
        {
            Assert.Empty(created.DirtyFields()); // freshly created model is clean
            created.SingleLineText = "dirty now";
            Assert.Single(created.DirtyFields());

            var saved = await created.SaveAsync();
            Assert.Empty(saved.DirtyFields()); // saved model is clean again
            Assert.Equal("dirty now", saved.SingleLineText);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task DeleteOnAnUnsavedModelThrows()
    {
        var model = new PrimaryModel { PrimaryKey = "never saved" };
        // Unsaved AND detached — the detached check fires first.
        await Assert.ThrowsAsync<AirtableException.ApiError>(() => model.DeleteAsync());
    }

    [Fact]
    public async Task FetchOnAnUnsavedModelThrows()
    {
        var model = new PrimaryModel { PrimaryKey = "never saved" };
        await Assert.ThrowsAsync<AirtableException.ApiError>(() => model.FetchAsync());
    }

    private async Task TryDeleteAsync(string? recordId)
    {
        if (string.IsNullOrEmpty(recordId))
            return;
        try
        {
            await _airtable.Primary.Orm.DeleteAsync(recordId);
        }
        catch (AirtableException)
        {
            // best-effort cleanup
        }
    }
}
