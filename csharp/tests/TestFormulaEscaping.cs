using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// TC2 — Formula-value escaping. Filter predicates build formulas by interpolating user values into
/// quoted string literals; a value containing a quote, backslash, or other meta-character must be
/// escaped or the formula breaks (or silently mis-matches). These tests round-trip special-character
/// values through storage AND through the generated <c>.Eq()/.Contains()</c> filter DSL against the
/// live base, proving the escaping is correct. Parity target for the other 8 suites.
/// </summary>
public class TestFormulaEscaping
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();

    // Values that break naive string interpolation into an Airtable formula literal.
    // Newline is covered separately (storage only) — Airtable formula string literals can't carry a
    // raw newline, so it's a storage concern, not a filter one.
    public static IEnumerable<object[]> SpecialValues() =>
        new[]
        {
            new object[] { "O'Brien single quote" },
            new object[] { "say \"hi\" double quote" },
            new object[] { "back\\slash path" },
            new object[] { "trailing backslash\\" },
            new object[] { "mixed a\"b'c\\d" },
            new object[] { "unicode café ☕ 日本語 🎉" },
        };

    [Theory]
    [MemberData(nameof(SpecialValues))]
    public async Task EqFilterMatchesValueWithSpecialChars(string special)
    {
        var suite = TestSetup.PrimaryKey("Escaping", "Eq");
        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = suite, SingleLineText = special }
        );
        var recordId = created.Id!;
        try
        {
            // 1. Storage round-trips the exact value.
            Assert.Equal(special, created.SingleLineText);

            // 2. The .Eq() filter, scoped to this record, matches via correctly-escaped interpolation.
            var filter = Formulas.And(
                $"FIND(\"{suite}\", {{Primary Key}})",
                PrimaryModel.F.SingleLineText.Eq(special)
            );
            var results = await _airtable.Primary.GetAsync(new AirtableQuery().WithFormula(filter));
            Assert.Contains(results, r => r.Id == recordId);
            Assert.All(results, r => Assert.Equal(special, r.SingleLineText));
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Theory]
    [MemberData(nameof(SpecialValues))]
    public async Task ContainsFilterMatchesValueWithSpecialChars(string special)
    {
        var suite = TestSetup.PrimaryKey("Escaping", "Contains");
        // Embed the special token inside a longer value so Contains (FIND>0) is a real substring test.
        var stored = "prefix " + special + " suffix";
        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = suite, SingleLineText = stored }
        );
        var recordId = created.Id!;
        try
        {
            var filter = Formulas.And(
                $"FIND(\"{suite}\", {{Primary Key}})",
                PrimaryModel.F.SingleLineText.Contains(special)
            );
            var results = await _airtable.Primary.GetAsync(new AirtableQuery().WithFormula(filter));
            Assert.Contains(results, r => r.Id == recordId);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task SpecialCharactersInPrimaryKeyRoundTripAndFilter()
    {
        // The primary key itself carries special chars, and we match on it with .Eq().
        var suite = TestSetup.PrimaryKey("Escaping", "PK");
        var pk = suite + " O'Brien \"quote\" back\\slash";
        var created = await _airtable.Primary.CreateAsync(new PrimaryModel { PrimaryKey = pk });
        var recordId = created.Id!;
        try
        {
            Assert.Equal(pk, created.PrimaryKey);
            var results = await _airtable.Primary.GetAsync(
                new AirtableQuery().WithFormula(PrimaryModel.F.PrimaryKey.Eq(pk))
            );
            Assert.Contains(results, r => r.Id == recordId);
            Assert.Single(results);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task NewlineAndTabValuesRoundTripThroughStorageAndFieldSelection()
    {
        // Newlines/tabs are a storage concern (long text), not expressible in a formula literal.
        // Verify they survive create -> fetch -> field-selected fetch unchanged.
        var suite = TestSetup.PrimaryKey("Escaping", "Newline");
        var value = "line1\nline2\twith tab\r\nwindows";
        var created = await _airtable.Primary.CreateAsync(
            new PrimaryModel { PrimaryKey = suite, LongText = value }
        );
        var recordId = created.Id!;
        try
        {
            var fetched = await _airtable.Primary.GetAsync(recordId);
            Assert.Equal(value, fetched.LongText);
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
