package myairtable.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;

import myairtable.MyAirtableRuntimeInfo;
import org.junit.jupiter.api.Test;

/** Proves the Gradle harness compiles the generated output/ and runs JUnit 5 tests. */
class TestHarness {
  @Test
  void generatedRuntimeIsOnTheClasspath() {
    assertEquals("0.0.1-dev", MyAirtableRuntimeInfo.VERSION);
  }
}
