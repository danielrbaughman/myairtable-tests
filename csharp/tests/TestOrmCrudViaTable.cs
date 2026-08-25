using System.Text.Json.Nodes;
using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// CS4.7 — Typed ORM CRUD via the table accessor (<c>airtable.Primary</c>). Parity with the
/// Java/Kotlin/Swift/Rust TestOrmCrudViaTable suites.
/// </summary>
public class TestOrmCrudViaTable
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();

    [Fact]
    public async Task PrimaryKeyOnlyCrudRoundTrip()
    {
        var primaryKey = TestSetup.PrimaryKey("OrmTable", "PKOnly");
        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = primaryKey }
        );
        Assert.False(string.IsNullOrEmpty(created.Id));
        Assert.Equal(primaryKey, created.PrimaryKey);

        var recordId = created.Id!;
        try
        {
            var fetched = await _airtable.Primary.GetAsync(recordId);
            Assert.Equal(recordId, fetched.Id);
            Assert.Equal(primaryKey, fetched.PrimaryKey);
            Assert.Empty(fetched.DirtyFields());

            fetched.PrimaryKey = primaryKey + " Updated";
            Assert.NotEmpty(fetched.DirtyFields());
            var updated = await _airtable.Primary.UpdateAsync(fetched);
            Assert.Equal(primaryKey + " Updated", updated.PrimaryKey);

            await _airtable.Primary.DeleteAsync(recordId);
            await Assert.ThrowsAnyAsync<AirtableException>(() =>
                _airtable.Primary.GetAsync(recordId)
            );
        }
        catch
        {
            await TryDeleteAsync(recordId);
            throw;
        }
    }

    [Fact]
    public async Task AllSimplePropertiesRoundTrip()
    {
        var primaryKey = TestSetup.PrimaryKey("OrmTable", "AllProps");
        var model = new PrimaryModel
        {
            PrimaryKey = primaryKey,
            SingleLineText = "Hello World",
            LongText = "Long text content",
            Email = "test@example.com",
            Url = "https://example.com",
            PhoneNumber = "555-1234",
            Checkbox = true,
            NumberInt = 42.0,
            NumberFloat = 3.14,
            CurrencyInt = 10.0,
            CurrencyFloat = 9.99,
            PercentInt = 0.5,
            PercentFloat = 0.333,
            Duration = TimeSpan.FromSeconds(3600),
        };
        var created = await _airtable.Primary.CreateAsync(model);
        var recordId = created.Id!;
        try
        {
            Assert.Equal(primaryKey, created.PrimaryKey);
            Assert.Equal("Hello World", created.SingleLineText);
            Assert.Equal("test@example.com", created.Email);
            Assert.True(created.Checkbox);
            Assert.Equal(42.0, created.NumberInt);
            Assert.Equal(3.14, created.NumberFloat);
            Assert.Equal(9.99, created.CurrencyFloat);
            Assert.Equal("https://example.com", created.Url);
            Assert.Equal("555-1234", created.PhoneNumber);
            Assert.Equal(TimeSpan.FromSeconds(3600), created.Duration); // duration wire unit is seconds
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task SelectEnumsAndRatingRoundTrip()
    {
        var primaryKey = TestSetup.PrimaryKey("OrmTable", "Selects");
        var model = new PrimaryModel
        {
            PrimaryKey = primaryKey,
            SingleSelect = PrimarySingleSelectOption.Choice1,
            MultipleSelect = new List<PrimaryMultipleSelectOption>
            {
                PrimaryMultipleSelectOption.Option1,
                PrimaryMultipleSelectOption.Option2,
            },
            Rating = 3L,
        };
        var created = await _airtable.Primary.CreateAsync(model);
        var recordId = created.Id!;
        try
        {
            Assert.Equal(PrimarySingleSelectOption.Choice1, created.SingleSelect);
            Assert.Equal(
                new List<PrimaryMultipleSelectOption>
                {
                    PrimaryMultipleSelectOption.Option1,
                    PrimaryMultipleSelectOption.Option2,
                },
                created.MultipleSelect
            );
            Assert.Equal(3L, created.Rating);

            var fetched = await _airtable.Primary.GetAsync(recordId);
            Assert.Equal(PrimarySingleSelectOption.Choice1, fetched.SingleSelect);

            fetched.SingleSelect = PrimarySingleSelectOption.Choice2;
            fetched.MultipleSelect = new List<PrimaryMultipleSelectOption>
            {
                PrimaryMultipleSelectOption.Option3,
            };
            fetched.Rating = 5L;
            var updated = await _airtable.Primary.UpdateAsync(fetched);
            Assert.Equal(PrimarySingleSelectOption.Choice2, updated.SingleSelect);
            Assert.Equal(
                new List<PrimaryMultipleSelectOption> { PrimaryMultipleSelectOption.Option3 },
                updated.MultipleSelect
            );
            Assert.Equal(5L, updated.Rating);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task GetQueryReturnsFilteredRecordSet()
    {
        var suite = TestSetup.PrimaryKey("OrmTable", "List");
        var models = Enumerable
            .Range(1, 3)
            .Select(i => new PrimaryModel { PrimaryKey = $"{suite} {i}" })
            .ToList();
        var created = await _airtable.Primary.CreateAsync(models);
        var createdIds = created.Select(m => m.Id!).ToList();
        try
        {
            Assert.Equal(3, created.Count);
            var results = await _airtable.Primary.GetAsync(
                new AirtableQuery().WithFormula($"FIND(\"{suite}\", {{Primary Key}})")
            );
            Assert.Equal(3, results.Count);
            Assert.Equal(
                new HashSet<string>(createdIds),
                new HashSet<string>(results.Select(m => m.Id!))
            );
        }
        finally
        {
            await TryDeleteManyAsync(createdIds);
        }
    }

    [Fact]
    public async Task FieldSelectionAndMaxRecordsApply()
    {
        var suite = TestSetup.PrimaryKey("OrmTable", "Select");
        var models = Enumerable
            .Range(1, 3)
            .Select(i => new PrimaryModel
            {
                PrimaryKey = $"{suite} {i}",
                SingleLineText = $"text {i}",
            })
            .ToList();
        var created = await _airtable.Primary.CreateAsync(models);
        var createdIds = created.Select(m => m.Id!).ToList();
        try
        {
            var results = await _airtable.Primary.GetAsync(
                new AirtableQuery()
                    .WithFormula($"FIND(\"{suite}\", {{Primary Key}})")
                    .WithFields(PrimaryFields.PrimaryKeyId)
                    .WithMaxRecords(2)
            );
            Assert.Equal(2, results.Count); // maxRecords caps the result set
            foreach (var model in results)
            {
                Assert.NotNull(model.PrimaryKey);
                Assert.Null(model.SingleLineText); // unselected fields decode as null
            }
        }
        finally
        {
            await TryDeleteManyAsync(createdIds);
        }
    }

    [Fact]
    public async Task InvalidRecordIdThrows()
    {
        await Assert.ThrowsAnyAsync<AirtableException>(() =>
            _airtable.Primary.GetAsync("recDOESNOTEXIST123")
        );
    }

    [Fact]
    public async Task BatchCreateUpdateDeleteChunksAboveTheBatchLimit()
    {
        var suite = TestSetup.PrimaryKey("OrmTable", "Batch");
        const int count = 111;
        var models = Enumerable
            .Range(1, count)
            .Select(i => new PrimaryModel { PrimaryKey = $"{suite} {i}" })
            .ToList();
        // CreateAsync chunks into 10-record batches; a mid-batch failure throws without returning
        // ids from earlier batches, so fall back to a prefix sweep (the suite prefix is unique).
        var createdIds = new List<string>();
        try
        {
            var created = await _airtable.Primary.CreateAsync(models);
            createdIds = created.Select(m => m.Id!).ToList();

            Assert.Equal(count, created.Count);
            for (var i = 0; i < created.Count; i++)
                Assert.Equal($"{suite} {i + 1}", created[i].PrimaryKey);

            foreach (var model in created)
                model.PrimaryKey += " Updated";
            var updated = await _airtable.Primary.UpdateAsync(created);
            Assert.Equal(count, updated.Count);
            for (var i = 0; i < updated.Count; i++)
                Assert.Equal($"{suite} {i + 1} Updated", updated[i].PrimaryKey);

            await _airtable.Primary.DeleteAsync(createdIds);
            var firstId = createdIds[0];
            await Assert.ThrowsAnyAsync<AirtableException>(() =>
                _airtable.Primary.GetAsync(firstId)
            );
        }
        catch
        {
            await TryDeleteManyAsync(createdIds);
            await TrySweepByPrefixAsync(suite);
            throw;
        }
    }

    [Fact]
    public async Task UpsertInsertsWhenNoMatchThenUpdatesOnMatch()
    {
        var primaryKey = TestSetup.PrimaryKey("OrmTable", "Upsert");
        var first = await _airtable.Primary.UpsertAsync(
            new PrimaryModel { PrimaryKey = primaryKey },
            new[] { PrimaryFields.PrimaryKeyId }
        );
        var recordId = first.Model.Id!;
        try
        {
            Assert.True(first.WasCreated); // no match yet — upsert inserts
            Assert.Equal(primaryKey, first.Model.PrimaryKey);

            var second = await _airtable.Primary.UpsertAsync(
                new PrimaryModel { PrimaryKey = primaryKey, SingleLineText = "merged" },
                new[] { PrimaryFields.PrimaryKeyId }
            );
            Assert.False(second.WasCreated); // matched on primary key — upsert updates
            Assert.Equal(recordId, second.Model.Id);
            Assert.Equal("merged", second.Model.SingleLineText);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    // ---- cleanup helpers ----

    private async Task TryDeleteAsync(string? recordId)
    {
        if (string.IsNullOrEmpty(recordId))
            return;
        try
        {
            await _airtable.Primary.DeleteAsync(recordId);
        }
        catch (AirtableException)
        {
            // best-effort cleanup
        }
    }

    private async Task TryDeleteManyAsync(IReadOnlyList<string> recordIds)
    {
        if (recordIds.Count == 0)
            return;
        try
        {
            await _airtable.Primary.DeleteAsync(recordIds);
        }
        catch (AirtableException)
        {
            // best-effort cleanup
        }
    }

    private async Task TrySweepByPrefixAsync(string suite)
    {
        try
        {
            var stray = await _airtable.Primary.GetAsync(
                new AirtableQuery().WithFormula($"FIND(\"{suite}\", {{Primary Key}})")
            );
            if (stray.Count > 0)
                await _airtable.Primary.DeleteAsync(stray.Select(m => m.Id!).ToList());
        }
        catch (AirtableException)
        {
            // best-effort cleanup
        }
    }
}
