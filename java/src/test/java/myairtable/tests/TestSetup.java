package myairtable.tests;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.time.Instant;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ThreadLocalRandom;
import myairtable.Airtable;

/**
 * Shared fixtures for Java integration tests.
 *
 * <p>Walks up from CWD looking for a .env file, parses simple KEY=VALUE lines (ignoring blank lines
 * and #-comments), and surfaces AIRTABLE_API_KEY + AIRTABLE_BASE_ID as helpers. Mirrors what Rust's
 * {@code dotenvy} and Python's conftest.py do for the other language suites. JVM processes can't
 * mutate their own environment, so values are kept in a map (System.getenv wins).
 */
public final class TestSetup {

  private static Map<String, String> dotEnv;

  private TestSetup() {}

  private static synchronized Map<String, String> dotEnv() {
    if (dotEnv == null) {
      dotEnv = loadDotEnv();
    }
    return dotEnv;
  }

  private static String env(String key) {
    String fromEnv = System.getenv(key);
    return fromEnv != null ? fromEnv : dotEnv().get(key);
  }

  /** Construct an Airtable wrapper from .env with caching disabled. */
  public static Airtable makeAirtable() {
    return makeAirtable(0.0);
  }

  /** Construct an Airtable wrapper from .env. Fails fast if vars are absent. */
  public static Airtable makeAirtable(double cacheSeconds) {
    String apiKey = env("AIRTABLE_API_KEY");
    String baseId = env("AIRTABLE_BASE_ID");
    if (apiKey == null || apiKey.isEmpty()) {
      throw new IllegalStateException(
          "AIRTABLE_API_KEY not set. Ensure .env is present in the test repo root.");
    }
    if (baseId == null || baseId.isEmpty()) {
      throw new IllegalStateException(
          "AIRTABLE_BASE_ID not set. Ensure .env is present in the test repo root.");
    }
    return new Airtable(baseId, apiKey, cacheSeconds);
  }

  /**
   * A client pointed at the real base but with a bogus API key, for auth-failure (401) tests.
   * Mirrors the C# {@code MakeAirtableWithBadKey} helper: the base id is real (from .env) so the
   * failure is authentication, not a missing base.
   */
  public static Airtable makeAirtableWithBadKey() {
    String baseId = env("AIRTABLE_BASE_ID");
    if (baseId == null || baseId.isEmpty()) {
      throw new IllegalStateException(
          "AIRTABLE_BASE_ID not set. Ensure .env is present in the test repo root.");
    }
    return new Airtable(baseId, "patBOGUS00000.deadbeefdeadbeefdeadbeefdeadbeef");
  }

  /**
   * Walk up from CWD looking for ".env", but never climb past the repository root so a stray ".env"
   * higher up the filesystem (e.g. in a parent workspace or $HOME) can't be silently adopted.
   *
   * <p>".env" is inspected in each directory (including the repo-root directory itself) before the
   * boundary is enforced — the test repo keeps its ".env" alongside the ".git" marker at the repo
   * root, so it is still found. The Gradle build run from {@code java/} is a nested build, so a
   * ".git" directory (the VCS root) is the authoritative boundary here; a bare nested {@code
   * settings.gradle.kts} that has no co-located ".env" must NOT stop the walk early, or the
   * repo-root ".env" one level up would be missed. Once the {@code .git} root is inspected without
   * a ".env", the walk fails safe (empty map) rather than escaping the repo.
   */
  private static Map<String, String> loadDotEnv() {
    File dir = new File(System.getProperty("user.dir")).getAbsoluteFile();
    while (dir != null) {
      File candidate = new File(dir, ".env");
      if (candidate.isFile()) {
        return parseDotEnv(candidate);
      }
      // Stop once we've inspected the repo-root directory: don't climb into foreign territory.
      if (isRepoRoot(dir)) {
        return Map.of();
      }
      dir = dir.getParentFile();
    }
    return Map.of();
  }

  /**
   * A directory is the repo boundary if it holds the VCS root marker ({@code .git}). A {@code
   * settings.gradle.kts} also marks a (sub-)project root, but only counts as the boundary when it
   * additionally carries the repo's ".env" — otherwise the nested Gradle build directory would halt
   * the walk before reaching the repo-root ".env" that sits one level above it.
   */
  private static boolean isRepoRoot(File dir) {
    if (new File(dir, ".git").exists()) {
      return true;
    }
    return new File(dir, "settings.gradle.kts").isFile() && new File(dir, ".env").isFile();
  }

  private static Map<String, String> parseDotEnv(File file) {
    Map<String, String> values = new HashMap<>();
    List<String> lines;
    try {
      lines = Files.readAllLines(file.toPath());
    } catch (IOException e) {
      return Map.of();
    }
    for (String rawLine : lines) {
      String line = rawLine.trim();
      if (line.isEmpty() || line.startsWith("#")) {
        continue;
      }
      int eq = line.indexOf('=');
      if (eq < 0) {
        continue;
      }
      String key = line.substring(0, eq).trim();
      String value = line.substring(eq + 1).trim();
      if (value.length() >= 2
          && ((value.startsWith("\"") && value.endsWith("\""))
              || (value.startsWith("'") && value.endsWith("'")))) {
        value = value.substring(1, value.length() - 1);
      }
      values.put(key, value);
    }
    return values;
  }

  /**
   * Unique primary key for a given test to avoid cross-test collisions. Each test suite should pick
   * a distinct prefix so parallel test files don't create records with conflicting keys.
   *
   * <p>An epoch-second timestamp alone collides when two suite runs start within the same second
   * (e.g. CI re-runs, or a local run overlapping a CI run against the shared base). A short random
   * suffix makes the key unique across simultaneous runs.
   */
  public static String primaryKey(String suite, String label) {
    String rand = Long.toString(ThreadLocalRandom.current().nextLong(0, 0x1_0000_0000L), 36);
    return "Java " + suite + " " + label + " " + Instant.now().getEpochSecond() + "-" + rand;
  }
}
