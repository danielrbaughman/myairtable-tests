package myairtable.tests;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.time.Instant;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
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

  /** Walk from CWD up to the filesystem root looking for ".env" and parse it. */
  private static Map<String, String> loadDotEnv() {
    File dir = new File(System.getProperty("user.dir")).getAbsoluteFile();
    while (dir != null) {
      File candidate = new File(dir, ".env");
      if (candidate.isFile()) {
        return parseDotEnv(candidate);
      }
      dir = dir.getParentFile();
    }
    return Map.of();
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
   */
  public static String primaryKey(String suite, String label) {
    return "Java " + suite + " " + label + " " + Instant.now().getEpochSecond();
  }
}
