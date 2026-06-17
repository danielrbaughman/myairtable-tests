using System.Globalization;
using System.Text.Json;
using System.Text.Json.Nodes;
using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// CS8.12 — Runtime formula parity: create a record with all input fields, fetch it back with
/// API-computed formula values, and verify the transpiled <c>Evaluate*()</c> methods reproduce
/// them. Parity with the Java/Kotlin/Swift/Rust runtime-formula suites.
/// </summary>
public class TestRuntimeFormulas
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();

    private static DateTimeOffset Utc(string s) =>
        DateTimeOffset.Parse(
            s,
            CultureInfo.InvariantCulture,
            DateTimeStyles.AssumeUniversal | DateTimeStyles.AdjustToUniversal
        );

    [Fact]
    public async Task RuntimeFormulasMatchApi()
    {
        var fresh = new FormulasModel
        {
            FirstDate = Utc("2024-01-01T00:00:00.000Z"),
            FirstNumber = 10.0,
            FirstText = "Hello",
            PrimaryKey = "CSharpRuntimeTest",
            SecondDate = Utc("2024-02-01T00:00:00.000Z"),
            SecondNumber = 20.0,
            SecondText = "World",
            ThirdDate = Utc("2024-03-01T00:00:00.000Z"),
            ThirdNumber = 30.0,
            ThirdText = "!",
        };
        var created = await _airtable.Formulas.Orm.CreateAsync(fresh);
        var recordId = created.Id!;
        try
        {
            // Re-fetch to get formula values computed by Airtable.
            var model = await _airtable.Formulas.Orm.GetAsync(recordId);

            // Math formula: exact match.
            var runtimeMath = AirtableRuntime.S(model.EvaluateMathFormula());
            Assert.Equal(model.MathFormula!.ValueOrDefault, runtimeMath);

            // Text formula: exact match.
            var runtimeText = AirtableRuntime.S(model.EvaluateTextFormula());
            Assert.Equal(model.TextFormula!.ValueOrDefault, runtimeText);

            // Date formula: deterministic parts (TODAY/TONOW/FROMNOW are time-dependent).
            var apiDate = model.DateFormula!.ValueOrDefault ?? "";
            var runtimeDate = AirtableRuntime.S(model.EvaluateDateFormula());

            Assert.Contains("YEAR: 2024", runtimeDate);
            Assert.Contains("MONTH: 1", runtimeDate);
            Assert.Contains("DAY: 1", runtimeDate);
            Assert.Contains("HOUR: 0", runtimeDate);
            Assert.Contains("MINUTE: 0", runtimeDate);
            Assert.Contains("SECOND: 0", runtimeDate);
            Assert.Contains("WORKDAY_DIFF: ", runtimeDate);
            Assert.Contains("DATEADD+2d: 2024-01-03", runtimeDate);
            Assert.Contains("IS_BEFORE: Yes", runtimeDate);
            Assert.Contains("IS_SAME day: No", runtimeDate);
            Assert.Contains("IS_AFTER: Yes", runtimeDate);
            Assert.Contains("DATESTR: ", runtimeDate);
            Assert.Contains("TIMESTR: ", runtimeDate);

            // Compare the deterministic prefix of API vs runtime (before TODAY: which varies).
            var apiPrefix = apiDate.Split(", TODAY:")[0];
            var runtimePrefix = runtimeDate.Split(", TODAY:")[0];
            Assert.Equal(apiPrefix, runtimePrefix);
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public void EvaluateRunsOffline()
    {
        // Offline smoke check: a model decoded from a synthetic fields object (with dates) still
        // evaluates every generated method without throwing.
        var fields = new JsonObject
        {
            [FormulasFields.PrimaryKeyId] = "RuntimeTest",
            [FormulasFields.FirstNumberId] = 10,
            [FormulasFields.SecondNumberId] = 20,
            [FormulasFields.ThirdNumberId] = 30,
            [FormulasFields.FirstTextId] = "Hello",
            [FormulasFields.SecondTextId] = "World",
            [FormulasFields.ThirdTextId] = "!",
            [FormulasFields.FirstDateId] = "2024-01-01",
            [FormulasFields.SecondDateId] = "2024-02-01",
            [FormulasFields.ThirdDateId] = "2024-03-01",
        };
        var model = fields.Deserialize<FormulasModel>(AirtableJson.Options)!;

        Assert.NotNull(model.EvaluateMathFormula());
        var text = AirtableRuntime.S(model.EvaluateTextFormula());
        Assert.Contains("Hello", text);
        Assert.Contains("World", text);
        var date = AirtableRuntime.S(model.EvaluateDateFormula());
        Assert.Contains("YEAR: 2024", date);
    }

    private async Task TryDeleteAsync(string? recordId)
    {
        if (string.IsNullOrEmpty(recordId))
            return;
        try
        {
            await _airtable.Formulas.Orm.DeleteAsync(recordId);
        }
        catch (AirtableException) { }
    }
}
