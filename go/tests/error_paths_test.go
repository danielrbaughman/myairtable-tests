package tests

// TC3 — Error/validation paths with TYPED errors. Mirrors the C# TestErrorPaths
// suite. Go surfaces errors as values, so we classify via errors.As into the
// generated *airtable.APIError (which carries StatusCode + Type, where Type is
// the Airtable error code such as NOT_FOUND) rather than asserting a bare
// err != nil. This proves the client classifies HTTP failures into specific
// typed errors instead of one opaque error. Parity target.

import (
	"context"
	"errors"
	"testing"
	"time"

	airtable "myairtabletests/output"
)

// newAirtableWithBadKey builds a client pointed at the REAL base but with a
// bogus PAT, so the auth (401) path is exercised rather than the base (404).
// Mirrors C# TestSetup.MakeAirtableWithBadKey.
func newAirtableWithBadKey(t *testing.T) *airtable.Airtable {
	t.Helper()
	requireCreds(t)
	return airtable.NewWithBase(bogusAPIKey, baseID(), 0)
}

// asAPIError asserts err is a typed *airtable.APIError and returns it. It fails
// the test (with the actual concrete type) when the error is not classified —
// the whole point of TC3 is that failures are typed, not opaque.
func asAPIError(t *testing.T, err error) *airtable.APIError {
	t.Helper()
	if err == nil {
		t.Fatal("expected an error, got nil")
	}
	var apiErr *airtable.APIError
	if !errors.As(err, &apiErr) {
		t.Fatalf("expected *airtable.APIError, got %T: %v", err, err)
	}
	return apiErr
}

func errorPathPrimaryFields(suite string) *airtable.Fields {
	fields := airtable.NewFields(nil, airtable.PrimaryNameToID)
	_ = fields.Set(airtable.PrimaryPrimaryKeyFieldID, suite)
	return fields
}

func TestErrorPaths(t *testing.T) {
	at := newAirtable(t)
	ctx := context.Background()

	t.Run("GetNonexistentRecord", func(t *testing.T) {
		_, err := at.PrimaryDict.GetOne(ctx, "recDOESNOTEXIST0001")
		api := asAPIError(t, err)
		t.Logf("404 -> *APIError{StatusCode:%d, Type:%q}: %v", api.StatusCode, api.Type, err)
		if api.StatusCode != 404 {
			t.Fatalf("expected 404, got %d", api.StatusCode)
		}
		// The 404 sentinel must also match for this typed error.
		if !errors.Is(err, airtable.ErrNotFound) {
			t.Fatalf("expected errors.Is(err, ErrNotFound) for a 404")
		}
		if api.Type != "NOT_FOUND" {
			t.Fatalf("expected code NOT_FOUND, got %q", api.Type)
		}
	})

	t.Run("CreateWithInvalidSelectOption", func(t *testing.T) {
		fields := errorPathPrimaryFields(primaryKey("Error", "BadSelect"))
		_ = fields.Set(airtable.PrimarySingleSelectFieldID, "NotARealOption_zzz")
		created, err := at.PrimaryDict.CreateOne(ctx, fields)
		if err == nil {
			at.PrimaryDict.DeleteOne(ctx, created.ID) //nolint:errcheck // cleanup unexpected success
			t.Fatal("expected an error creating with an invalid select option")
		}
		api := asAPIError(t, err)
		t.Logf("bad select -> *APIError{StatusCode:%d, Type:%q}: %v", api.StatusCode, api.Type, err)
		if api.Type != "INVALID_MULTIPLE_CHOICE_OPTIONS" {
			t.Fatalf("expected code INVALID_MULTIPLE_CHOICE_OPTIONS, got %q", api.Type)
		}
	})

	t.Run("CreateWithWrongValueType", func(t *testing.T) {
		fields := errorPathPrimaryFields(primaryKey("Error", "BadType"))
		_ = fields.Set(airtable.PrimaryNumberIntFieldID, "not a number")
		created, err := at.PrimaryDict.CreateOne(ctx, fields)
		if err == nil {
			at.PrimaryDict.DeleteOne(ctx, created.ID) //nolint:errcheck // cleanup unexpected success
			t.Fatal("expected an error creating with a wrong-typed value")
		}
		api := asAPIError(t, err)
		t.Logf("wrong type -> *APIError{StatusCode:%d, Type:%q}: %v", api.StatusCode, api.Type, err)
		if api.Type != "INVALID_VALUE_FOR_COLUMN" {
			t.Fatalf("expected code INVALID_VALUE_FOR_COLUMN, got %q", api.Type)
		}
	})

	t.Run("CreateWithUnknownField", func(t *testing.T) {
		fields := errorPathPrimaryFields(primaryKey("Error", "BadField"))
		_ = fields.Set("fldDOESNOTEXIST00000", "x")
		created, err := at.PrimaryDict.CreateOne(ctx, fields)
		if err == nil {
			at.PrimaryDict.DeleteOne(ctx, created.ID) //nolint:errcheck // cleanup unexpected success
			t.Fatal("expected an error creating with an unknown field id")
		}
		api := asAPIError(t, err)
		t.Logf("unknown field -> *APIError{StatusCode:%d, Type:%q}: %v", api.StatusCode, api.Type, err)
		if api.Type != "UNKNOWN_FIELD_NAME" {
			t.Fatalf("expected code UNKNOWN_FIELD_NAME, got %q", api.Type)
		}
	})

	t.Run("BadApiKeyIsAuthError", func(t *testing.T) {
		bad := newAirtableWithBadKey(t)
		_, err := bad.PrimaryDict.GetMany(ctx, (&airtable.Query{}).WithMaxRecords(1))
		api := asAPIError(t, err)
		t.Logf("bad key -> *APIError{StatusCode:%d, Type:%q}: %v", api.StatusCode, api.Type, err)
		if api.StatusCode != 401 {
			t.Fatalf("expected 401, got %d", api.StatusCode)
		}
		if api.Type != "AUTHENTICATION_REQUIRED" {
			t.Fatalf("expected code AUTHENTICATION_REQUIRED, got %q", api.Type)
		}
	})

	t.Run("RateLimitedErrorIsDistinctType", func(t *testing.T) {
		// Go exposes a distinct *airtable.RateLimitError type (429, retries
		// exhausted) carrying its RetryAfter. The live 429 path is covered by
		// TC8; here we assert the type is constructible, distinct from APIError,
		// and carries its Retry-After value.
		err := error(&airtable.RateLimitError{RetryAfter: 30 * time.Second})
		var rl *airtable.RateLimitError
		if !errors.As(err, &rl) {
			t.Fatalf("expected *airtable.RateLimitError, got %T", err)
		}
		if rl.RetryAfter != 30*time.Second {
			t.Fatalf("expected RetryAfter 30s, got %s", rl.RetryAfter)
		}
		// Distinct from APIError — must NOT classify as one.
		var api *airtable.APIError
		if errors.As(err, &api) {
			t.Fatal("RateLimitError must be distinct from APIError")
		}
	})
}
