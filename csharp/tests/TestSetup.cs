using MyAirtable;

namespace MyAirtable.Tests;

/// <summary>Shared integration-test helpers: .env discovery, client factory, unique keys.</summary>
public static class TestSetup
{
    private static readonly Lazy<Dictionary<string, string>> DotEnv = new(LoadDotEnv);

    private static string? Env(string key) =>
        Environment.GetEnvironmentVariable(key) ?? DotEnv.Value.GetValueOrDefault(key);

    public static Airtable MakeAirtable(double cacheSeconds = 0)
    {
        var apiKey = Env("AIRTABLE_API_KEY");
        var baseId = Env("AIRTABLE_BASE_ID");
        if (string.IsNullOrEmpty(apiKey))
            throw new InvalidOperationException(
                "AIRTABLE_API_KEY not set (need .env in the test repo root)."
            );
        if (string.IsNullOrEmpty(baseId))
            throw new InvalidOperationException(
                "AIRTABLE_BASE_ID not set (need .env in the test repo root)."
            );
        return new Airtable(baseId, apiKey, cacheSeconds);
    }

    /// <summary>A client pointed at the real base but with a bogus API key, for auth-failure tests.</summary>
    public static Airtable MakeAirtableWithBadKey()
    {
        var baseId = Env("AIRTABLE_BASE_ID");
        if (string.IsNullOrEmpty(baseId))
            throw new InvalidOperationException(
                "AIRTABLE_BASE_ID not set (need .env in the test repo root)."
            );
        return new Airtable(baseId, "patBOGUS00000.deadbeefdeadbeefdeadbeefdeadbeef");
    }

    /// <summary>A primary-key value unique to this run (distinct per file via <paramref name="suite"/>).</summary>
    public static string PrimaryKey(string suite, string label)
    {
        var ts = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();
        var rand = Random.Shared.Next(100000, 999999);
        return $"CSharp {suite} {label} {ts}-{rand}";
    }

    private static Dictionary<string, string> LoadDotEnv()
    {
        var dir = new DirectoryInfo(Directory.GetCurrentDirectory());
        while (dir is not null)
        {
            var candidate = Path.Combine(dir.FullName, ".env");
            if (File.Exists(candidate))
                return ParseDotEnv(candidate);
            dir = dir.Parent;
        }
        return new Dictionary<string, string>();
    }

    private static Dictionary<string, string> ParseDotEnv(string path)
    {
        var map = new Dictionary<string, string>();
        foreach (var raw in File.ReadAllLines(path))
        {
            var line = raw.Trim();
            if (line.Length == 0 || line.StartsWith('#'))
                continue;
            var eq = line.IndexOf('=');
            if (eq <= 0)
                continue;
            var key = line[..eq].Trim();
            var value = line[(eq + 1)..].Trim().Trim('"', '\'');
            map[key] = value;
        }
        return map;
    }
}
