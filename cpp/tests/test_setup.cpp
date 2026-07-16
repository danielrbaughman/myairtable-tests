#include "test_setup.hpp"

#include <chrono>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <map>
#include <random>
#include <sstream>
#include <stdexcept>

// The generated runtime is header-only except the vendored tz implementation,
// which must be compiled in exactly one TU — this is that TU (stb-style).
// Guarded so the skeleton compiles before the first build.sh run.
#if __has_include("static/airtable_tz.hpp")
#define MYAIRTABLE_TZ_IMPLEMENTATION
#include "static/airtable_tz.hpp"
#endif

namespace myairtable_tests {

namespace {

std::map<std::string, std::string> parse_dotenv(const std::filesystem::path& path) {
    std::map<std::string, std::string> values;
    std::ifstream in(path);
    std::string line;
    while (std::getline(in, line)) {
        if (line.empty() || line[0] == '#') {
            continue;
        }
        const auto eq = line.find('=');
        if (eq == std::string::npos) {
            continue;
        }
        std::string key = line.substr(0, eq);
        std::string value = line.substr(eq + 1);
        // Trim whitespace and a single layer of quotes.
        const auto trim = [](std::string& s) {
            const auto notspace = [](unsigned char c) { return !std::isspace(c); };
            s.erase(s.begin(), std::find_if(s.begin(), s.end(), notspace));
            s.erase(std::find_if(s.rbegin(), s.rend(), notspace).base(), s.end());
            if (s.size() >= 2 && ((s.front() == '"' && s.back() == '"') ||
                                  (s.front() == '\'' && s.back() == '\''))) {
                s = s.substr(1, s.size() - 2);
            }
        };
        trim(key);
        trim(value);
        if (!key.empty()) {
            values[key] = value;
        }
    }
    return values;
}

const std::map<std::string, std::string>& dotenv() {
    static const std::map<std::string, std::string> values = [] {
        auto dir = std::filesystem::current_path();
        while (true) {
            const auto candidate = dir / ".env";
            if (std::filesystem::exists(candidate)) {
                return parse_dotenv(candidate);
            }
            if (dir == dir.root_path()) {
                return std::map<std::string, std::string>{};
            }
            dir = dir.parent_path();
        }
    }();
    return values;
}

} // namespace

std::string env(const std::string& key) {
    if (const char* value = std::getenv(key.c_str()); value != nullptr && *value != '\0') {
        return value;
    }
    const auto& values = dotenv();
    const auto it = values.find(key);
    return it == values.end() ? std::string{} : it->second;
}

std::string require_api_key() {
    auto value = env("AIRTABLE_API_KEY");
    if (value.empty()) {
        throw std::runtime_error("AIRTABLE_API_KEY not set (need .env in the test repo root).");
    }
    return value;
}

std::string require_base_id() {
    auto value = env("AIRTABLE_BASE_ID");
    if (value.empty()) {
        throw std::runtime_error("AIRTABLE_BASE_ID not set (need .env in the test repo root).");
    }
    return value;
}

std::string primary_key(const std::string& suite, const std::string& label) {
    const auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(
                        std::chrono::system_clock::now().time_since_epoch())
                        .count();
    static std::mt19937 rng{std::random_device{}()};
    std::uniform_int_distribution<int> dist(100000, 999999);
    std::ostringstream out;
    out << "Cpp " << suite << " " << label << " " << ms << "-" << dist(rng);
    return out.str();
}

} // namespace myairtable_tests
