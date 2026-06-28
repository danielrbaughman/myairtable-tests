using System.Globalization;
using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// TC7 — Field-type round-trip completeness via re-fetch. Several field types were only asserted on
/// the create response (or only offline-decoded), never written and read back through the live API,
/// and clearing/removing multi-value fields was untested. Each case here creates, optionally updates,
/// re-fetches, and asserts the server-side value. Parity target for the other 8 suites.
/// </summary>
public class TestFieldRoundTrip
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();

    private static DateTimeOffset Utc(string s) =>
        DateTimeOffset.Parse(
            s,
            CultureInfo.InvariantCulture,
            DateTimeStyles.AssumeUniversal | DateTimeStyles.AdjustToUniversal
        );

    private const string UserId = "usrnZ4k98m0Ipji4e"; // shared-base user, as in TestComplexProperties

    [Fact]
    public async Task DateWithTimeWritesAndReadsBack()
    {
        var suite = TestSetup.PrimaryKey("FieldRT", "DateTime");
        var dt = Utc("2024-03-15T14:30:00.000Z");
        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = suite, DateWithTime = dt }
        );
        var recordId = created.Id!;
        try
        {
            var fetched = await _airtable.Primary.GetAsync(recordId);
            Assert.Equal(dt, fetched.DateWithTime);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task RichTextAndPercentCurrencyReadBack()
    {
        var suite = TestSetup.PrimaryKey("FieldRT", "Rich");
        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel
            {
                PrimaryKey = suite,
                LongTextWithRichText = "**bold** and _italic_ text",
                PercentInt = 0.5,
                PercentFloat = 0.333,
                CurrencyInt = 100.0,
                CurrencyFloat = 19.99,
            }
        );
        var recordId = created.Id!;
        try
        {
            var fetched = await _airtable.Primary.GetAsync(recordId);
            Assert.Equal("**bold** and _italic_ text", fetched.LongTextWithRichText);
            Assert.Equal(0.5, fetched.PercentInt);
            Assert.Equal(0.333, fetched.PercentFloat);
            Assert.Equal(100.0, fetched.CurrencyInt);
            Assert.Equal(19.99, fetched.CurrencyFloat);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task ClearingSingleAndMultiSelectReadsBackEmpty()
    {
        var suite = TestSetup.PrimaryKey("FieldRT", "ClearSelect");
        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel
            {
                PrimaryKey = suite,
                SingleSelect = PrimarySingleSelectOption.Choice1,
                MultipleSelect = new List<PrimaryMultipleSelectOption>
                {
                    PrimaryMultipleSelectOption.Option1,
                    PrimaryMultipleSelectOption.Option2,
                },
            }
        );
        var recordId = created.Id!;
        try
        {
            Assert.Equal(PrimarySingleSelectOption.Choice1, created.SingleSelect);

            created.SingleSelect = null;
            created.MultipleSelect = new List<PrimaryMultipleSelectOption>();
            await _airtable.Primary.UpdateAsync(created);

            var fetched = await _airtable.Primary.GetAsync(recordId);
            Assert.Null(fetched.SingleSelect);
            Assert.True(fetched.MultipleSelect is null or { Count: 0 });
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task RemovingACollaboratorReadsBackNull()
    {
        var suite = TestSetup.PrimaryKey("FieldRT", "RemoveUser");
        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel
            {
                PrimaryKey = suite,
                User = new AirtableCollaborator { Id = UserId },
            }
        );
        var recordId = created.Id!;
        try
        {
            Assert.Equal(UserId, created.User!.Id);

            created.User = null;
            await _airtable.Primary.UpdateAsync(created);

            var fetched = await _airtable.Primary.GetAsync(recordId);
            Assert.Null(fetched.User);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task AttachmentReplaceAndRemoveReadBack()
    {
        const string urlA =
            "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png";
        const string urlB = "https://www.w3.org/Icons/w3c_home.png";
        var suite = TestSetup.PrimaryKey("FieldRT", "Attach");
        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel
            {
                PrimaryKey = suite,
                Attachment = new List<AirtableAttachment> { new() { Url = urlA } },
            }
        );
        var recordId = created.Id!;
        try
        {
            // Replace the attachment with a different one.
            created.Attachment = new List<AirtableAttachment> { new() { Url = urlB } };
            await _airtable.Primary.UpdateAsync(created);
            var replaced = await _airtable.Primary.GetAsync(recordId);
            Assert.NotNull(replaced.Attachment);
            Assert.Single(replaced.Attachment!);

            // Remove all attachments.
            replaced.Attachment = new List<AirtableAttachment>();
            await _airtable.Primary.UpdateAsync(replaced);
            var cleared = await _airtable.Primary.GetAsync(recordId);
            Assert.True(cleared.Attachment is null or { Count: 0 });
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
        catch (AirtableException) { }
    }
}
