// Cache thread-safety (C# TestCacheConcurrency parity, 2 cases). The TTL cache
// is shared mutable state on the client; the caching suite only exercises it
// single-threaded. These fire concurrent reads and concurrent mutations at one
// cached client and assert no corruption/exception and a consistent result.
// (Run the cpp_tests_tsan variant to surface unsynchronized access.)
#include <catch2/catch_test_macros.hpp>

#include <exception>
#include <string>
#include <thread>
#include <vector>

#include "test_setup.hpp"

using namespace myairtable;
using myairtable_tests::make_airtable;
using myairtable_tests::primary_key;

namespace {

Airtable cached() {
    return make_airtable(60.0);
}

void best_effort_delete(Airtable& at, const std::string& id) {
    if (id.empty()) {
        return;
    }
    try {
        at.primary().delete_one(id);
    } catch (const AirtableException&) {
    }
}

/// Join every thread, then rethrow the first captured exception (if any).
void join_and_rethrow(std::vector<std::thread>& threads, std::vector<std::exception_ptr>& errors) {
    for (auto& t : threads) {
        t.join();
    }
    for (const auto& error : errors) {
        if (error) {
            std::rethrow_exception(error);
        }
    }
}

} // namespace

TEST_CASE("cache concurrency: concurrent gets of the same record are consistent",
          "[cache][cache-concurrency]") {
    auto at = cached();
    const auto key = primary_key("CacheConc", "Reads");
    auto created = at.primary().create_one(PrimaryModel{.primary_key = key});
    const auto id = *created.id;
    try {
        // 25 concurrent gets race to populate/read the shared cache.
        constexpr int kReaders = 25;
        std::vector<PrimaryModel> results(kReaders);
        std::vector<std::exception_ptr> errors(kReaders);
        std::vector<std::thread> threads;
        threads.reserve(kReaders);
        for (int i = 0; i < kReaders; ++i) {
            threads.emplace_back([&, i] {
                try {
                    results[i] = at.primary().get_one(id);
                } catch (...) {
                    errors[i] = std::current_exception();
                }
            });
        }
        join_and_rethrow(threads, errors);

        for (const auto& model : results) {
            REQUIRE(model.primary_key == key);
            REQUIRE(model.id == id);
        }
    } catch (...) {
        best_effort_delete(at, id);
        throw;
    }
    best_effort_delete(at, id);
}

TEST_CASE("cache concurrency: concurrent reads and mutations do not corrupt the cache",
          "[cache][cache-concurrency]") {
    auto at = cached();
    const auto key = primary_key("CacheConc", "Mix");
    auto created = at.primary().create_one(PrimaryModel{.primary_key = key});
    const auto id = *created.id;
    try {
        // Interleave reads (populate cache), manual invalidations, and
        // re-reads concurrently.
        constexpr int kIterations = 15;
        std::vector<std::exception_ptr> errors(kIterations * 3);
        std::vector<std::thread> threads;
        threads.reserve(kIterations * 3);
        for (int i = 0; i < kIterations; ++i) {
            threads.emplace_back([&, slot = i * 3] {
                try {
                    at.primary().get_one(id);
                } catch (...) {
                    errors[slot] = std::current_exception();
                }
            });
            threads.emplace_back([&, slot = i * 3 + 1] {
                try {
                    at.client()->invalidate_cache(std::string(PrimaryModel::kTableId));
                } catch (...) {
                    errors[slot] = std::current_exception();
                }
            });
            threads.emplace_back([&, slot = i * 3 + 2] {
                try {
                    at.invalidate_all_caches();
                } catch (...) {
                    errors[slot] = std::current_exception();
                }
            });
        }
        join_and_rethrow(threads, errors);

        // The client is still usable and returns the correct value afterward.
        auto fetched = at.primary().get_one(id);
        REQUIRE(fetched.primary_key == key);
    } catch (...) {
        best_effort_delete(at, id);
        throw;
    }
    best_effort_delete(at, id);
}
