using System.Text.Json;
using System.Text.Json.Nodes;
using MyAirtable;
using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// CS4.6 — Offline (no-network) model serialization parity with the Java/Kotlin/Swift/Rust
/// TestSerializing suites. Decodes a record's <c>fields</c> object (keyed by field id) into the
/// generated <see cref="PrimaryModel"/>, and exercises the writable-only payload builders +
/// snapshot/dirty machinery. Network-bound cases live in the F10 serialization scenario file.
/// </summary>
public class TestSerializing
{
    private static PrimaryModel Decode(JsonObject fields) =>
        fields.Deserialize<PrimaryModel>(AirtableJson.Options)!;

    // ---- decode ----

    [Fact]
    public void DecodePrimaryFieldsKeyedByFieldId()
    {
        var model = Decode(
            new JsonObject
            {
                [PrimaryFields.PrimaryKeyId] = "PK Value",
                [PrimaryFields.SingleLineTextId] = "hello",
                [PrimaryFields.NumberIntId] = 42,
                [PrimaryFields.CheckboxId] = true,
            }
        );
        Assert.Equal("PK Value", model.PrimaryKey);
        Assert.Equal("hello", model.SingleLineText);
        Assert.Equal(42.0, model.NumberInt);
        Assert.True(model.Checkbox);
    }

    [Fact]
    public void DateOnlyAndDatetimeFieldValuesBothDecode()
    {
        var model = Decode(
            new JsonObject
            {
                [PrimaryFields.DateId] = "2024-01-15",
                [PrimaryFields.DateWithTimeId] = "2024-01-15T10:30:00.000Z",
            }
        );
        Assert.Equal(new DateTimeOffset(2024, 1, 15, 0, 0, 0, TimeSpan.Zero), model.Date);
        Assert.Equal(new DateTimeOffset(2024, 1, 15, 10, 30, 0, TimeSpan.Zero), model.DateWithTime);
    }

    [Fact]
    public void MissingFieldsAreNull()
    {
        var model = Decode(new JsonObject { [PrimaryFields.PrimaryKeyId] = "only pk" });
        Assert.Equal("only pk", model.PrimaryKey);
        Assert.Null(model.SingleLineText);
        Assert.Null(model.NumberInt);
        Assert.Null(model.Date);
    }

    [Fact]
    public void EmptyFieldsObjectDecodesCleanly()
    {
        var model = Decode(new JsonObject());
        Assert.Null(model.PrimaryKey);
        Assert.True(model.IsNew);
    }

    // ---- encode / payload builders ----

    [Fact]
    public void EncodeProducesFieldsKeyedByFieldId()
    {
        var model = new PrimaryModel { PrimaryKey = "x", NumberInt = 7.0 };
        var record = model.ToCreateFields();
        Assert.Equal(
            new HashSet<string> { PrimaryFields.PrimaryKeyId, PrimaryFields.NumberIntId },
            new HashSet<string>(record.Keys)
        );
        Assert.Equal("x", record[PrimaryFields.PrimaryKeyId]!.GetValue<string>());
    }

    [Fact]
    public void ToRecordEmitsFieldValuesKeyedByFieldId()
    {
        var model = new PrimaryModel { PrimaryKey = "rec", Email = "a@b.c" };
        var record = model.ToRecord();
        Assert.Equal("rec", record[PrimaryFields.PrimaryKeyId]!.GetValue<string>());
        Assert.Equal("a@b.c", record[PrimaryFields.EmailId]!.GetValue<string>());
    }

    [Fact]
    public void RoundTripPreservesAllWritableFieldValues()
    {
        var original = new PrimaryModel
        {
            PrimaryKey = "rt",
            SingleLineText = "text",
            Checkbox = true,
            NumberInt = 5.0,
            NumberFloat = 2.5,
        };
        var encoded = JsonSerializer.Serialize(original, AirtableJson.Options);
        var decoded = JsonSerializer.Deserialize<PrimaryModel>(encoded, AirtableJson.Options)!;
        Assert.Equal(original.PrimaryKey, decoded.PrimaryKey);
        Assert.Equal(original.SingleLineText, decoded.SingleLineText);
        Assert.Equal(original.Checkbox, decoded.Checkbox);
        Assert.Equal(original.NumberInt, decoded.NumberInt);
        Assert.Equal(original.NumberFloat, decoded.NumberFloat);
    }

    [Fact]
    public void FreshModelOmitsNullFieldsSparseWrite()
    {
        var model = new PrimaryModel { PrimaryKey = "sparse" };
        var record = model.ToCreateFields();
        Assert.Equal(
            new HashSet<string> { PrimaryFields.PrimaryKeyId },
            new HashSet<string>(record.Keys)
        );
        // STJ encode also skips nulls (WhenWritingNull) incl. the [JsonInclude] computed props.
        var encoded = JsonSerializer.Serialize(model, AirtableJson.Options);
        Assert.Equal($"{{\"{PrimaryFields.PrimaryKeyId}\":\"sparse\"}}", encoded);
    }

    [Fact]
    public void RatingIsATypedInteger()
    {
        // `rating` maps to GenericType.INTEGER, so it is a real long rather than the
        // type-erased JsonNode it used to be. It still serializes as a JSON number.
        var model = new PrimaryModel { PrimaryKey = "r", Rating = 3L };
        Assert.Equal(3L, model.Rating);
        Assert.Equal(3L, model.ToCreateFields()[PrimaryFields.RatingId]!.GetValue<long>());
    }

    // ---- snapshot / dirty tracking ----

    [Fact]
    public void FreshDecodeHasNoDirtyFieldsAfterSnapshot()
    {
        var model = Decode(new JsonObject { [PrimaryFields.PrimaryKeyId] = "clean" });
        model.TakeSnapshot();
        Assert.Empty(model.DirtyFields());
    }

    [Fact]
    public void MutatingAWritableFieldMarksOnlyThatFieldDirty()
    {
        var model = Decode(
            new JsonObject
            {
                [PrimaryFields.PrimaryKeyId] = "pk",
                [PrimaryFields.SingleLineTextId] = "before",
            }
        );
        model.TakeSnapshot();
        model.SingleLineText = "after";
        Assert.Equal(
            new HashSet<string> { PrimaryFields.SingleLineTextId },
            new HashSet<string>(model.DirtyFields().Keys)
        );
    }

    [Fact]
    public void TakeSnapshotClearsTheDirtySet()
    {
        var model = Decode(new JsonObject { [PrimaryFields.PrimaryKeyId] = "pk" });
        model.TakeSnapshot();
        model.SingleLineText = "mutated";
        Assert.NotEmpty(model.DirtyFields());
        model.TakeSnapshot();
        Assert.Empty(model.DirtyFields());
    }

    [Fact]
    public void ClearedWritableFieldBecomesJsonNull()
    {
        var model = Decode(
            new JsonObject
            {
                [PrimaryFields.PrimaryKeyId] = "pk",
                [PrimaryFields.NumberIntId] = 9.5,
            }
        );
        model.TakeSnapshot();
        model.NumberInt = null;
        var dirty = model.DirtyFields();
        Assert.True(dirty.ContainsKey(PrimaryFields.NumberIntId));
        Assert.Null(dirty[PrimaryFields.NumberIntId]); // JSON null = C# null JsonNode
    }

    [Fact]
    public void ShrinkingAMultiValueFieldIsDetectedAsDirty()
    {
        // Regression: dirty tracking must use structural equality, not formula-style coercion
        // (which compares arrays by their first element only and would miss [a, b] -> [a]).
        var model = new PrimaryModel
        {
            Id = "rec1",
            LinkMultiple = new List<string> { "recA", "recB" },
        };
        model.TakeSnapshot();
        model.LinkMultiple = new List<string> { "recA" };
        var dirty = model.DirtyFields();
        Assert.True(dirty.ContainsKey(PrimaryFields.LinkMultipleId));
        Assert.Equal("recA", dirty[PrimaryFields.LinkMultipleId]!.AsArray()[0]!.GetValue<string>());
        Assert.Single(dirty[PrimaryFields.LinkMultipleId]!.AsArray());
    }

    // ---- identity ----

    [Fact]
    public void UnsavedModelReportsIsNewTrue()
    {
        var model = new PrimaryModel { PrimaryKey = "new" };
        Assert.True(model.IsNew);
        Assert.Null(model.Id);
    }

    [Fact]
    public void ModelWithIdReportsIsNewFalse()
    {
        var model = new PrimaryModel { PrimaryKey = "saved", Id = "recXYZ" };
        Assert.False(model.IsNew);
    }
}
