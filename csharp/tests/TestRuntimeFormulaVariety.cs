using System.Globalization;
using MyAirtable;
using Xunit;
using Xunit.Abstractions;

namespace MyAirtable.Tests;

/// <summary>
/// TC4 — Runtime-formula input variety. The base TestRuntimeFormulas suite only evaluates the
/// kitchen-sink formulas with ONE fully-populated input set, so the IF(OR(...=BLANK())) short-circuit
/// and varied inputs are never exercised. Each case here creates a Formulas record with a specific
/// input set, fetches the API-computed value, and asserts the transpiled Evaluate*() reproduces it.
///
/// Scope notes (kept deliberately portable across all 9 targets):
///  • Numbers are positive with LOG/SQRT arguments chosen to be exact (powers of 10 / perfect
///    squares). The Math formula calls LOG()/SQRT()/EXP() — transcendental results differ by a ULP
///    between platforms' math libs and Airtable (V8), so irrational results (e.g. LOG(5)) are NOT
///    bit-identical and would make an exact string compare flaky. Negatives/zero are excluded too:
///    LOG(negative) and MOD(_,0) error inside this formula. See myairtable-5b0n.
///  • Text covers empty-ish edges (whitespace), unicode, and reserved punctuation (exercising the
///    fixed ENCODE_URL_COMPONENT). An all-blank text input is excluded: Airtable returns blank for
///    the whole formula (REPLACE past end-of-string errors), while the transpiler is lenient.
/// </summary>
public class TestRuntimeFormulaVariety
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();
    private readonly ITestOutputHelper _out;

    public TestRuntimeFormulaVariety(ITestOutputHelper output) => _out = output;

    private static DateTimeOffset Utc(string s) =>
        DateTimeOffset.Parse(
            s,
            CultureInfo.InvariantCulture,
            DateTimeStyles.AssumeUniversal | DateTimeStyles.AdjustToUniversal
        );

    private static FormulasModel Base(string label) =>
        new()
        {
            PrimaryKey = "CSharp Variety " + label,
            FirstDate = Utc("2024-01-01T00:00:00.000Z"),
            SecondDate = Utc("2024-02-01T00:00:00.000Z"),
            ThirdDate = Utc("2024-03-01T00:00:00.000Z"),
        };

    // First Number: power of 10 (LOG exact). Second Number: perfect square (SQRT exact).
    public static IEnumerable<object[]> NumberCases() =>
        new[]
        {
            new object[] { "hundreds", 100.0, 16.0, 8.0 },
            new object[] { "ones", 1.0, 4.0, 2.0 },
            new object[] { "tens", 10.0, 25.0, 3.0 },
        };

    [Theory]
    [MemberData(nameof(NumberCases))]
    public async Task MathFormulaMatchesApiForVariedNumbers(
        string label,
        double a,
        double b,
        double c
    )
    {
        var model = Base("Math " + label);
        model.FirstNumber = a;
        model.SecondNumber = b;
        model.ThirdNumber = c;
        model.FirstText = "x";
        model.SecondText = "y";
        model.ThirdText = "z";
        var created = await _airtable.Formulas.CreateAsync(model);
        try
        {
            var fetched = await _airtable.Formulas.GetAsync(created.Id!);
            var runtime = AirtableRuntime.S(fetched.EvaluateMathFormula());
            _out.WriteLine(
                $"{label}: api='{fetched.MathFormula?.ValueOrDefault}' runtime='{runtime}'"
            );
            Assert.Equal(fetched.MathFormula!.ValueOrDefault, runtime);
        }
        finally
        {
            await TryDeleteAsync(created.Id);
        }
    }

    [Fact]
    public async Task MathFormulaBlankBranchWhenNumbersMissing()
    {
        // First/Second Number left null -> OR(BLANK, BLANK) is true -> the formula returns BLANK().
        // This is the IF-true short-circuit that the base suite never reaches.
        var model = Base("Blank");
        model.FirstText = "x";
        model.SecondText = "y";
        model.ThirdText = "z";
        var created = await _airtable.Formulas.CreateAsync(model);
        try
        {
            var fetched = await _airtable.Formulas.GetAsync(created.Id!);
            var apiVal = fetched.MathFormula?.ValueOrDefault;
            var runtime = AirtableRuntime.S(fetched.EvaluateMathFormula());
            _out.WriteLine($"blank: api='{apiVal}' runtime='{runtime}'");
            Assert.True(string.IsNullOrEmpty(apiVal), $"API expected blank, got '{apiVal}'");
            Assert.True(string.IsNullOrEmpty(runtime), $"runtime expected blank, got '{runtime}'");
        }
        finally
        {
            await TryDeleteAsync(created.Id);
        }
    }

    public static IEnumerable<object[]> TextCases() =>
        new[]
        {
            new object[] { "unicode", "café", "naïve", "日本語🎉" },
            new object[] { "whitespace", "  he llo  ", "a b", "c" },
            new object[] { "punct", "a.e-i+o", "x/y", "z" }, // exercises fixed ENCODE_URL_COMPONENT
        };

    [Theory]
    [MemberData(nameof(TextCases))]
    public async Task TextFormulaMatchesApiForVariedText(string label, string a, string b, string c)
    {
        var model = Base("Text " + label);
        model.FirstNumber = 10.0;
        model.SecondNumber = 20.0;
        model.ThirdNumber = 30.0;
        model.FirstText = a;
        model.SecondText = b;
        model.ThirdText = c;
        var created = await _airtable.Formulas.CreateAsync(model);
        try
        {
            var fetched = await _airtable.Formulas.GetAsync(created.Id!);
            var runtime = AirtableRuntime.S(fetched.EvaluateTextFormula());
            _out.WriteLine(
                $"{label}: api='{fetched.TextFormula?.ValueOrDefault}' runtime='{runtime}'"
            );
            Assert.Equal(fetched.TextFormula!.ValueOrDefault, runtime);
        }
        finally
        {
            await TryDeleteAsync(created.Id);
        }
    }

    private async Task TryDeleteAsync(string? recordId)
    {
        if (string.IsNullOrEmpty(recordId))
            return;
        try
        {
            await _airtable.Formulas.DeleteAsync(recordId);
        }
        catch (AirtableException) { }
    }
}
