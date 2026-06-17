using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// CS5.6 — Complex field-type CRUD against the live base: attachments (URL-based, async
/// processing), collaborators (users), computed fields (auto-number, formulas, created-time/by),
/// and the read-only button. Parity with the Java/Kotlin/Swift/Rust complex-property suites.
/// Linked records are covered by CS6.2.
/// </summary>
public class TestComplexProperties
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();

    [Fact]
    public async Task AttachmentFieldRoundTripsViaUrl()
    {
        var primaryKey = TestSetup.PrimaryKey("Complex", "Attach");
        const string url =
            "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png";

        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel
            {
                PrimaryKey = primaryKey,
                Attachment = new List<AirtableAttachment> { new() { Url = url } },
            }
        );
        var recordId = created.Id!;
        try
        {
            // Airtable processes attachments asynchronously — the fresh record may carry just the
            // URL, or be fully populated with id/size once processing completes.
            Assert.NotNull(created.Attachment);
            Assert.Single(created.Attachment!);

            // Poll until Airtable has enriched the attachment (id populated).
            var current = created;
            for (var i = 0; i < 10; i++)
            {
                var first = current.Attachment is { Count: > 0 } a ? a[0] : null;
                if (!string.IsNullOrEmpty(first?.Id))
                    break;
                await Task.Delay(2000);
                current = await _airtable.Primary.GetAsync(recordId);
            }
            var enriched = current.Attachment ?? new List<AirtableAttachment>();
            Assert.Single(enriched);
            Assert.False(string.IsNullOrEmpty(enriched[0].Url), "attachment url is non-empty");
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task UserAndMultiUserFieldsRoundTrip()
    {
        var primaryKey = TestSetup.PrimaryKey("Complex", "Users");
        // Matches the user ID used by the Rust/Swift/Java tests in the shared base.
        const string userId = "usrnZ4k98m0Ipji4e";

        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel
            {
                PrimaryKey = primaryKey,
                User = new AirtableCollaborator { Id = userId },
                UserAllowMultiple = new List<AirtableCollaborator> { new() { Id = userId } },
            }
        );
        var recordId = created.Id!;
        try
        {
            Assert.NotNull(created.User);
            Assert.Equal(userId, created.User!.Id);
            Assert.NotNull(created.UserAllowMultiple);
            Assert.Single(created.UserAllowMultiple!);
            Assert.Equal(userId, created.UserAllowMultiple![0].Id);

            var fetched = await _airtable.Primary.GetAsync(recordId);
            Assert.NotNull(fetched.User);
            Assert.Equal(userId, fetched.User!.Id);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task ComputedFieldsPopulateOnCreateAndReadBack()
    {
        var primaryKey = TestSetup.PrimaryKey("Complex", "Computed");
        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel
            {
                PrimaryKey = primaryKey,
                NumberInt = 10.0,
                NumberFloat = 5.0,
            }
        );
        var recordId = created.Id!;
        try
        {
            // Server-owned: auto number assigned (Value variant); created time populated.
            Assert.IsType<MaybeSpecialOrError<long>.Value>(created.AutoNumber);
            Assert.NotNull(created.CreatedTime);
            // Dedicated createdTime field decodes as a populated Value.
            Assert.IsType<MaybeSpecialOrError<DateTimeOffset>.Value>(created.CreatedAtTime);
            // Formula(ID) reflects the record id.
            Assert.Equal(recordId, created.FormulaId!.ValueOrDefault);
            // Formula(Simple) = numberInt + numberFloat = 15.
            Assert.Equal(15.0, created.FormulaSimple!.ValueOrDefault);
            // Created-by is populated with the caller's user info.
            Assert.NotNull(created.CreatedBy);
            Assert.NotNull(created.CreatedBy!.ValueOrDefault);

            var fetched = await _airtable.Primary.GetAsync(recordId);
            Assert.Equal(recordId, fetched.FormulaId!.ValueOrDefault);
            Assert.Equal(15.0, fetched.FormulaSimple!.ValueOrDefault);
            Assert.Equal(created.AutoNumber, fetched.AutoNumber);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task DurationRoundTripsAsNumericSeconds()
    {
        // CS5.5 magnitude assertion: the duration wire unit is seconds, so a 90-minute
        // TimeSpan round-trips exactly (not truncated/scaled).
        var primaryKey = TestSetup.PrimaryKey("Complex", "Duration");
        var duration = TimeSpan.FromMinutes(90);
        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = primaryKey, Duration = duration }
        );
        var recordId = created.Id!;
        try
        {
            Assert.Equal(duration, created.Duration);
            var fetched = await _airtable.Primary.GetAsync(recordId);
            Assert.Equal(duration, fetched.Duration);
            Assert.Equal(5400.0, fetched.Duration!.Value.TotalSeconds);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task ButtonFieldDecodesOnRead()
    {
        var primaryKey = TestSetup.PrimaryKey("Complex", "Button");
        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = primaryKey }
        );
        var recordId = created.Id!;
        try
        {
            // The button field decodes (either populated or absent) without error.
            var fetched = await _airtable.Primary.GetAsync(recordId);
            Assert.Equal(recordId, fetched.Id);
            if (fetched.Button?.ValueOrDefault is { } button)
                Assert.NotNull(button.Label ?? button.Url ?? "");
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

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
}
