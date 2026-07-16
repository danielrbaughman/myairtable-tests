// Harness smoke tests: prove the CMake wiring, the generated-output include
// path, credential discovery, and Catch2's declaration-order execution — the
// mechanism the ordered live CRUD phases rely on (no orderer/fixture files, in
// contrast to the xUnit/JUnit harnesses).
#include <catch2/catch_test_macros.hpp>

#include "test_setup.hpp"

#if __has_include("static/my_airtable_runtime_info.hpp")
#include "static/my_airtable_runtime_info.hpp"
#define MYAIRTABLE_OUTPUT_PRESENT 1
#endif

namespace {
int order_probe = 0;
}

TEST_CASE("harness: generated output is on the include path", "[harness]") {
#ifdef MYAIRTABLE_OUTPUT_PRESENT
    REQUIRE_FALSE(myairtable::kVersion.empty());
#else
    SKIP("cpp/output not generated yet — run ../build.sh");
#endif
}

TEST_CASE("harness: credentials resolve from env or .env", "[harness]") {
    REQUIRE_FALSE(myairtable_tests::require_api_key().empty());
    REQUIRE_FALSE(myairtable_tests::require_base_id().empty());
}

TEST_CASE("harness: unique primary keys carry suite, label, and entropy", "[harness]") {
    const auto a = myairtable_tests::primary_key("Harness", "Uniq");
    const auto b = myairtable_tests::primary_key("Harness", "Uniq");
    REQUIRE(a.rfind("Cpp Harness Uniq ", 0) == 0);
    REQUIRE(a != b);
}

// Declaration-order proof (plan MED-3): Catch2 executes TEST_CASEs within one
// translation unit top-to-bottom. Phase 2 observing phase 1's side effect is
// exactly the shared-state pattern the live CRUD suites use.
TEST_CASE("harness: declaration order phase 1", "[harness][order]") {
    REQUIRE(order_probe == 0);
    order_probe = 1;
}

TEST_CASE("harness: declaration order phase 2", "[harness][order]") {
    REQUIRE(order_probe == 1);
}
