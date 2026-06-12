package myairtable.tests

import myairtable.Airtable
import java.io.File
import java.time.Instant

/**
 * Shared fixtures for Kotlin integration tests.
 *
 * Walks up from CWD looking for a .env file, parses simple KEY=VALUE lines
 * (ignoring blank lines and #-comments), and surfaces AIRTABLE_API_KEY +
 * AIRTABLE_BASE_ID as helpers. Mirrors what Rust's `dotenvy` and Python's
 * conftest.py do for the other language suites. JVM processes can't mutate
 * their own environment, so values are kept in a map (System.getenv wins).
 */
object TestSetup {
    private val dotEnv: Map<String, String> by lazy { loadDotEnv() }

    private fun env(key: String): String? = System.getenv(key) ?: dotEnv[key]

    /** Construct an Airtable wrapper from .env. Fails fast if vars are absent. */
    fun makeAirtable(cacheSeconds: Double = 0.0): Airtable {
        val apiKey = env("AIRTABLE_API_KEY")
        val baseId = env("AIRTABLE_BASE_ID")
        check(!apiKey.isNullOrEmpty()) { "AIRTABLE_API_KEY not set. Ensure .env is present in the test repo root." }
        check(!baseId.isNullOrEmpty()) { "AIRTABLE_BASE_ID not set. Ensure .env is present in the test repo root." }
        return Airtable(baseId = baseId, apiKey = apiKey, cacheSeconds = cacheSeconds)
    }

    /** Walk from CWD up to the filesystem root looking for ".env" and parse it. */
    private fun loadDotEnv(): Map<String, String> {
        var dir: File? = File(System.getProperty("user.dir")).absoluteFile
        while (dir != null) {
            val candidate = File(dir, ".env")
            if (candidate.isFile) return parseDotEnv(candidate)
            dir = dir.parentFile
        }
        return emptyMap()
    }

    private fun parseDotEnv(file: File): Map<String, String> =
        buildMap {
            for (rawLine in file.readLines()) {
                val line = rawLine.trim()
                if (line.isEmpty() || line.startsWith("#")) continue
                val eq = line.indexOf('=')
                if (eq < 0) continue
                val key = line.substring(0, eq).trim()
                var value = line.substring(eq + 1).trim()
                if (value.length >= 2 &&
                    ((value.startsWith("\"") && value.endsWith("\"")) || (value.startsWith("'") && value.endsWith("'")))
                ) {
                    value = value.substring(1, value.length - 1)
                }
                put(key, value)
            }
        }

    /**
     * Unique primary key for a given test to avoid cross-test collisions.
     * Each test suite should pick a distinct prefix so parallel test files
     * don't create records with conflicting keys.
     */
    fun primaryKey(
        suite: String,
        label: String,
    ): String = "Kotlin $suite $label ${Instant.now().epochSecond}"
}
