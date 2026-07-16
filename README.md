# myairtable-tests

Automated integration testing for myAirtable across all generated language targets.

Run `./test.sh <lang> [suite]` to regenerate code and run tests for a language,
or `./test.sh all` for every language. Supported `<lang>`: `ts`, `js`, `py`,
`rs`, `swift`, `kotlin`, `java`, `go`, `cs`, `cpp`.

C++ requires CMake ≥ 3.25 (`brew install cmake ninja`); the suite builds one
sequential Catch2 binary in `cpp/build/` against the generated `cpp/output/`,
and the `--cache` suite runs a ThreadSanitizer variant (the Go `-race` analog).
