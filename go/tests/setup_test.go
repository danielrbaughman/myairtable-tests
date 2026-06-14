package tests

// Shared integration-test harness: .env discovery, credential access, and a
// per-suite unique primary-key generator. Mirrors the Java TestSetup helper.

import (
	"bufio"
	"fmt"
	"math/rand"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
	"time"
)

var (
	envOnce sync.Once
	envVars map[string]string
)

// loadEnv walks up from the working directory to the repo root (the directory
// containing .git) looking for a .env file, parses KEY=VALUE lines (ignoring
// comments/blanks), and caches the result. Missing file -> empty map (the
// process environment is still consulted by getenv).
func loadEnv() map[string]string {
	envOnce.Do(func() {
		envVars = map[string]string{}
		dir, err := os.Getwd()
		if err != nil {
			return
		}
		for {
			candidate := filepath.Join(dir, ".env")
			if data, err := os.Open(candidate); err == nil {
				scanner := bufio.NewScanner(data)
				for scanner.Scan() {
					line := strings.TrimSpace(scanner.Text())
					if line == "" || strings.HasPrefix(line, "#") {
						continue
					}
					key, val, ok := strings.Cut(line, "=")
					if !ok {
						continue
					}
					envVars[strings.TrimSpace(key)] = strings.Trim(strings.TrimSpace(val), `"'`)
				}
				data.Close()
				break
			}
			if _, err := os.Stat(filepath.Join(dir, ".git")); err == nil {
				break // reached repo root without a .env
			}
			parent := filepath.Dir(dir)
			if parent == dir {
				break // filesystem root
			}
			dir = parent
		}
	})
	return envVars
}

// getenv returns the value for key, preferring the process environment and
// falling back to the discovered .env file.
func getenv(key string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return loadEnv()[key]
}

func apiKey() string { return getenv("AIRTABLE_API_KEY") }
func baseID() string { return getenv("AIRTABLE_BASE_ID") }

// requireCreds skips the test if live credentials are not configured.
func requireCreds(t *testing.T) {
	t.Helper()
	if apiKey() == "" || baseID() == "" {
		t.Skip("AIRTABLE_API_KEY / AIRTABLE_BASE_ID not set; skipping live test")
	}
}

// primaryKey returns a unique primary-key string for a suite/label, distinct
// per run, so concurrent files and reruns never collide on shared records.
func primaryKey(suite, label string) string {
	return fmt.Sprintf("%s %s %d-%d", suite, label, time.Now().Unix(), rand.Intn(1<<31))
}
