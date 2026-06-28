using System.Text.Json.Nodes;
using MyAirtable;
using Xunit;
using Xunit.Abstractions;

namespace MyAirtable.Tests;

/// <summary>
/// TC3 — Error and validation paths with TYPED exceptions (not bare catch-all). Exercises the
/// generated exception hierarchy against the live base: a 404 fetch, server-side validation
/// rejections (invalid select option, wrong value type, unknown field), and a 401 from a bad key.
/// Each asserts a specific generated exception subtype + its structured fields, proving the client
/// classifies HTTP failures rather than collapsing them to one opaque error. Parity target.
/// </summary>
public class TestErrorPaths
{
    private readonly Airtable _airtable = TestSetup.MakeAirtable();
    private readonly ITestOutputHelper _out;

    public TestErrorPaths(ITestOutputHelper output) => _out = output;

    private static Fields PrimaryFieldsWith(string suite)
    {
        var fields = new Fields(new Dictionary<string, JsonNode?>(), PrimaryFields.NameToId);
        fields.SetString(PrimaryFields.PrimaryKeyId, suite);
        return fields;
    }

    [Fact]
    public async Task GetNonexistentRecordThrowsApiError()
    {
        var ex = await Assert.ThrowsAnyAsync<AirtableException>(() =>
            _airtable.Primary.GetAsync("recDOESNOTEXIST0001")
        );
        _out.WriteLine($"404 -> {ex.GetType().Name}: {ex.Message}");
        var api = Assert.IsType<AirtableException.ApiError>(ex);
        Assert.False(string.IsNullOrEmpty(api.Code));
    }

    [Fact]
    public async Task CreateWithInvalidSelectOptionThrowsApiError()
    {
        // The typed enum can't express an invalid option, so go through the dict accessor.
        var fields = PrimaryFieldsWith(TestSetup.PrimaryKey("Error", "BadSelect"));
        fields.SetString(PrimaryFields.SingleSelectId, "NotARealOption_zzz");
        var ex = await Assert.ThrowsAnyAsync<AirtableException>(() =>
            _airtable.Primary.Dict.CreateAsync(fields)
        );
        _out.WriteLine($"bad select -> {ex.GetType().Name}: {ex.Message}");
        var api = Assert.IsType<AirtableException.ApiError>(ex);
        Assert.False(string.IsNullOrEmpty(api.Code));
    }

    [Fact]
    public async Task CreateWithWrongValueTypeThrowsApiError()
    {
        var fields = PrimaryFieldsWith(TestSetup.PrimaryKey("Error", "BadType"));
        fields.Set(PrimaryFields.NumberIntId, JsonValue.Create("not a number"));
        var ex = await Assert.ThrowsAnyAsync<AirtableException>(() =>
            _airtable.Primary.Dict.CreateAsync(fields)
        );
        _out.WriteLine($"wrong type -> {ex.GetType().Name}: {ex.Message}");
        Assert.IsType<AirtableException.ApiError>(ex);
    }

    [Fact]
    public async Task CreateWithUnknownFieldThrowsApiError()
    {
        var fields = new Fields(new Dictionary<string, JsonNode?>(), PrimaryFields.NameToId);
        fields.SetString(PrimaryFields.PrimaryKeyId, TestSetup.PrimaryKey("Error", "BadField"));
        fields.Set("fldDOESNOTEXIST00000", JsonValue.Create("x"));
        var ex = await Assert.ThrowsAnyAsync<AirtableException>(() =>
            _airtable.Primary.Dict.CreateAsync(fields)
        );
        _out.WriteLine($"unknown field -> {ex.GetType().Name}: {ex.Message}");
        Assert.IsType<AirtableException.ApiError>(ex);
    }

    [Fact]
    public async Task BadApiKeyThrowsAuthError()
    {
        // Real base + bogus key so auth (401) is checked, not the base (404).
        var bad = TestSetup.MakeAirtableWithBadKey();
        var ex = await Assert.ThrowsAnyAsync<AirtableException>(() =>
            bad.Primary.GetAsync(new AirtableQuery().WithMaxRecords(1))
        );
        _out.WriteLine($"bad key -> {ex.GetType().Name}: {ex.Message}");
        // Auth failures come back as a structured 401 envelope (ApiError) or, if not parseable,
        // a raw HttpError — both are acceptable typed classifications (never the base type alone).
        Assert.True(
            ex is AirtableException.ApiError or AirtableException.HttpError,
            $"expected ApiError/HttpError, got {ex.GetType().Name}"
        );
    }

    [Fact]
    public void RateLimitedErrorCarriesRetryAfter()
    {
        // The 429 path is exercised live in TC8; here we assert the type is a distinct, well-formed
        // member of the hierarchy carrying its Retry-After value.
        var err = new AirtableException.RateLimitedError(30.0);
        Assert.IsAssignableFrom<AirtableException>(err);
        Assert.Equal(30.0, err.RetryAfterSeconds);
    }
}
