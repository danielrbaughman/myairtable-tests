package tests

// Smoke test: proves the generated package compiles and is importable, and that
// the static-runtime version marker is on the classpath. Mirrors Java TestHarness.

import (
	"testing"

	airtable "myairtabletests/output"
)

func TestHarness(t *testing.T) {
	if airtable.VERSION == "" {
		t.Fatal("airtable.VERSION is empty; generated package not wired correctly")
	}
}
