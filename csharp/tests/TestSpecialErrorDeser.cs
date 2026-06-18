using System.Text.Json;
using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// CS5.7 (+ CS5.3 end-to-end) — parity with rust test_special_error_deser / Kotlin/Swift/Java
/// TestSpecialErrorDeser. Offline decoding of Airtable's <c>{"specialValue": ...}</c> /
/// <c>{"error": ...}</c> computed-field shapes through the generated module, plus a generated-model
/// decode proving erroring/special formula + lookup/rollup fields don't break deserialization.
/// </summary>
public class TestSpecialErrorDeser
{
    private static T Decode<T>(string raw) =>
        JsonSerializer.Deserialize<T>(raw, AirtableJson.Options)!;

    [Fact]
    public void SpecialNumberAcceptsObjectFormRejectsArrayForm()
    {
        Assert.Equal("NaN", Decode<SpecialNumber>("""{"specialValue":"NaN"}""").SpecialValue);
        Assert.ThrowsAny<Exception>(() => Decode<SpecialNumber>("""["NaN"]"""));
    }

    [Fact]
    public void ErrorValueAcceptsObjectFormRejectsArrayForm()
    {
        Assert.Equal("#ERROR!", Decode<ErrorValue>("""{"error":"#ERROR!"}""").Error);
        Assert.ThrowsAny<Exception>(() => Decode<ErrorValue>("""["#ERROR!"]"""));
    }

    [Fact]
    public void MaybeSpecialOrErrorStringParsesPlainStringRejectsLookupArray()
    {
        var plain = Decode<MaybeSpecialOrError<string>>("\"Stukenholtz\"");
        Assert.Equal("Stukenholtz", plain.ValueOrDefault);
        // Pre-fix regression in other targets: ["Stukenholtz"] accidentally succeeded as Special.
        Assert.ThrowsAny<Exception>(() =>
            Decode<MaybeSpecialOrError<string>>("""["Stukenholtz"]""")
        );
    }

    [Fact]
    public void VecOrValueLookupShapes()
    {
        var single = Decode<VecOrValue<MaybeSpecialOrError<string>>>("\"hello\"");
        var singleShape = Assert.IsType<VecOrValue<MaybeSpecialOrError<string>>.Single>(single);
        Assert.Equal("hello", singleShape.Value!.ValueOrDefault);

        var multiple = Decode<VecOrValue<MaybeSpecialOrError<string>>>(
            """["Stukenholtz Laboratory Inc"]"""
        );
        Assert.Equal(
            new List<string> { "Stukenholtz Laboratory Inc" },
            VecOrValue.CleanValues(multiple)
        );

        var withNulls = Decode<VecOrValue<MaybeSpecialOrError<string>>>("""["a", null, "b"]""");
        var multipleShape = Assert.IsType<VecOrValue<MaybeSpecialOrError<string>>.Multiple>(
            withNulls
        );
        Assert.Equal(3, multipleShape.Values.Count);
        Assert.Null(multipleShape.Values[1]);

        var special = Decode<VecOrValue<MaybeSpecialOrError<double>>>("""{"specialValue":"NaN"}""");
        var specialValue = Assert.IsType<MaybeSpecialOrError<double>.Special>(
            Assert.IsType<VecOrValue<MaybeSpecialOrError<double>>.Single>(special).Value
        );
        Assert.Equal("NaN", specialValue.Number.SpecialValue);

        var error = Decode<VecOrValue<MaybeSpecialOrError<double>>>("""{"error":"#ERROR!"}""");
        var errorValue = Assert.IsType<MaybeSpecialOrError<double>.Error>(
            Assert.IsType<VecOrValue<MaybeSpecialOrError<double>>.Single>(error).Value
        );
        Assert.Equal("#ERROR!", errorValue.Err.Error);
    }

    [Fact]
    public void ModelDecodesErroringComputedFields()
    {
        // The exact failure mode that used to throw in other targets: Airtable returns
        // error/special objects for computed fields, while lookup returns an array with nulls.
        var fieldsJson = $$"""
            {
              "{{PrimaryFields.PrimaryKeyId}}": "ErrDeser",
              "{{PrimaryFields.FormulaSimpleId}}": {"error": "#ERROR!"},
              "{{PrimaryFields.FormulaIdId}}": "recERRDESER123456",
              "{{PrimaryFields.LookupId}}": ["Lab A", null],
              "{{PrimaryFields.RollupId}}": {"specialValue": "NaN"}
            }
            """;
        var model = JsonSerializer.Deserialize<PrimaryModel>(fieldsJson, AirtableJson.Options)!;

        Assert.Equal("ErrDeser", model.PrimaryKey);
        // Error variant — ValueOrDefault is the non-nullable double default (0) for value-type T,
        // so the variant type IS the assertion.
        Assert.IsType<MaybeSpecialOrError<double>.Error>(model.FormulaSimple);
        Assert.Equal("recERRDESER123456", model.FormulaId!.ValueOrDefault);
        Assert.Equal(new List<string> { "Lab A" }, VecOrValue.CleanValues(model.Lookup));
        var lookupShape = Assert.IsType<VecOrValue<MaybeSpecialOrError<string>>.Multiple>(
            model.Lookup
        );
        Assert.Equal(2, lookupShape.Values.Count);
        var rollupSpecial = Assert.IsType<MaybeSpecialOrError<string>.Special>(
            Assert.IsType<VecOrValue<MaybeSpecialOrError<string>>.Single>(model.Rollup).Value
        );
        Assert.Equal("NaN", rollupSpecial.Number.SpecialValue);
    }
}
