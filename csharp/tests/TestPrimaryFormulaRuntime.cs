using MyAirtable;
using Xunit;
using Xunit.Abstractions;

namespace MyAirtable.Tests;

/// <summary>
/// TC6 — Primary "Formula (Complex)"/"Formula (Nested)" evaluated at runtime. The base runtime suite
/// only covers the Formulas table; the Primary complex formula concatenates ~35 fields through
/// IF(field, field, "None"), a richer transpile path never checked against the API before.
///
/// We compare the transpiled EvaluateFormulaComplex() to the API line-by-line for the DETERMINISTIC,
/// offline-reproducible field types (text, checkbox, single/multi select, numbers, currency, email,
/// url, phone). The formula also references server-computed fields (Created/Last Modified Time + By,
/// Auto Number, Button, Formula(ID)/(Simple)) and link/lookup/rollup — Airtable renders those from
/// data the offline runtime doesn't hold (collaborator names, linked-record display values, special
/// wrappers), so those lines are NOT expected to match offline. See myairtable-5b0n.
///
/// This suite specifically locks in the multi-select array-join fix (myairtable-bb7f): a multi-value
/// field coerces to "Option 1, Option 2", not just the first element.
/// </summary>
public class TestPrimaryFormulaRuntime
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();
    private readonly ITestOutputHelper _out;

    public TestPrimaryFormulaRuntime(ITestOutputHelper output) => _out = output;

    // Field labels whose rendering the offline runtime can reproduce exactly.
    private static readonly string[] DeterministicLabels =
    {
        "Single Line Text",
        "Long Text",
        "Checkbox",
        "Multiple Select",
        "Single Select",
        "Number (int)",
        "Number (float)",
        "Currency (int)",
        "Currency (float)",
        "Email",
        "URL",
        "Phone Number",
    };

    private PrimaryModel NewRecord(string suite) =>
        new()
        {
            PrimaryKey = suite,
            SingleLineText = "hello",
            LongText = "long text",
            Email = "a@b.co",
            Url = "https://x.co",
            PhoneNumber = "555-1212",
            Checkbox = true,
            NumberInt = 42.0,
            NumberFloat = 3.5,
            CurrencyInt = 10.0,
            CurrencyFloat = 9.99,
            SingleSelect = PrimarySingleSelectOption.Choice1,
            MultipleSelect = new List<PrimaryMultipleSelectOption>
            {
                PrimaryMultipleSelectOption.Option1,
                PrimaryMultipleSelectOption.Option2,
            },
        };

    /// <summary>Extract the "Label: value" line for <paramref name="label"/> from a formula result.</summary>
    private static string Line(string formula, string label)
    {
        foreach (var line in formula.Split('\n'))
            if (line.StartsWith(label + ": ", StringComparison.Ordinal))
                return line;
        return $"<missing: {label}>";
    }

    [Fact]
    public async Task ComplexFormulaRendersDeterministicFieldsLikeApi()
    {
        var suite = TestSetup.PrimaryKey("PrimaryFormula", "Complex");
        var created = await _airtable.Primary.CreateAsync(NewRecord(suite));
        var recordId = created.Id!;
        try
        {
            var fetched = await _airtable.Primary.GetAsync(recordId);
            var api = VecOrValue.CleanValues(fetched.FormulaComplex).FirstOrDefault() ?? "";
            var runtime = AirtableRuntime.S(fetched.EvaluateFormulaComplex());
            _out.WriteLine($"--- API ---\n{api}\n--- RUNTIME ---\n{runtime}");

            foreach (var label in DeterministicLabels)
                Assert.Equal(Line(api, label), Line(runtime, label));

            // The multi-select join is the headline fix: both sides render all options, comma-joined.
            Assert.Equal("Multiple Select: Option 1, Option 2", Line(runtime, "Multiple Select"));
        }
        finally
        {
            await TryDeleteAsync(recordId);
        }
    }

    [Fact]
    public async Task NestedFormulaEvaluatesWithoutThrowing()
    {
        // Formula (Nested) = Formula(ID) & Formula(Simple) & Formula(Complex) — it chains three
        // COMPUTED formula fields. Offline the runtime can't reproduce computed-field values (they
        // decode as special/wrapped types it doesn't re-evaluate), so the content isn't asserted;
        // this confirms the transpiled nested-formula method is generated and evaluates without
        // error. See myairtable-5b0n.
        var suite = TestSetup.PrimaryKey("PrimaryFormula", "Nested");
        var created = await _airtable.Primary.CreateAsync(NewRecord(suite));
        var recordId = created.Id!;
        try
        {
            var fetched = await _airtable.Primary.GetAsync(recordId);
            var runtime = AirtableRuntime.S(fetched.EvaluateFormulaNested()); // must not throw
            Assert.NotNull(runtime);
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
