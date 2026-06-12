package myairtable.tests

import myairtable.MyAirtableRuntimeInfo
import kotlin.test.Test
import kotlin.test.assertEquals

/** Proves the Gradle harness compiles the generated output/ and runs JUnit 5 tests. */
class TestHarness {
    @Test
    fun generatedRuntimeIsOnTheClasspath() {
        assertEquals("0.0.1-dev", MyAirtableRuntimeInfo.VERSION)
    }
}
