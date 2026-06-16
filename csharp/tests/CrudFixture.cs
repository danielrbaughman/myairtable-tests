namespace MyAirtable.Tests;

/// <summary>
/// Shared mutable state for a sequenced CRUD test class (injected via
/// <c>IClassFixture&lt;CrudFixture&gt;</c>). Carries record IDs created in early phases so
/// later phases (read/update/delete) can reference them — xUnit builds a fresh test-class
/// instance per method, so per-instance fields cannot carry this state.
///
/// F1 skeleton: holds the created-ID map. The Airtable-client-backed cleanup in
/// <see cref="Dispose"/> is wired once the generated client exists (F3+).
/// </summary>
public sealed class CrudFixture : IDisposable
{
    /// <summary>Logical-label -> created record ID, populated across ordered phases.</summary>
    public Dictionary<string, string> CreatedIds { get; } = new();

    public void Dispose()
    {
        // Best-effort cleanup of any records the test class failed to delete is added in F3+
        // once MyAirtable.Airtable is generated.
        CreatedIds.Clear();
    }
}
