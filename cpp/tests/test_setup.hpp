#pragma once

// Shared integration-test helpers: .env discovery, credentials, unique keys.
// The client factory (make_airtable) is added once the generated Airtable
// entry point exists (F3) — everything here is generated-code-independent.

#include <string>

namespace myairtable_tests {

/// Environment lookup: real environment first, then the nearest `.env` walking
/// up from the current working directory (same discovery as every other
/// language's TestSetup). Returns "" when unset.
std::string env(const std::string& key);

/// AIRTABLE_API_KEY / AIRTABLE_BASE_ID, throwing with a setup hint when missing.
std::string require_api_key();
std::string require_base_id();

/// A primary-key value unique to this run: "Cpp {suite} {label} {ms}-{rand}".
/// Distinct per file via `suite` so parallel/repeated runs never collide and
/// prefix sweeps are safe.
std::string primary_key(const std::string& suite, const std::string& label);

} // namespace myairtable_tests
