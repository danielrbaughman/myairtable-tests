#!/bin/bash
set -e

# Python
uv sync
uv run ruff check
uv run ty check
uv run ruff format

# TypeScript / JavaScript
if ! command -v nvm &> /dev/null; then
    export NVM_DIR="$HOME/.nvm"
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
fi
nvm use
yarn install
yarn lint
yarn format

# Rust
cargo check
cargo fmt

# Swift
if command -v swift &> /dev/null; then
    (cd swift && swift build)
    if command -v swift-format &> /dev/null; then
        # Format the test tree; the generated output/ is regenerated on every build
        # and swift-format would just re-format it on the next run anyway.
        swift-format format --in-place --recursive --configuration .swift-format swift/Tests
    else
        echo "[warn] swift-format not installed; skipping format step. (brew install swift-format)"
    fi
else
    echo "[warn] Swift not on PATH; skipping Swift checks."
fi

# Kotlin
if [ -x kotlin/gradlew ] && command -v java &> /dev/null; then
    (cd kotlin && ./gradlew compileTestKotlin)
    if command -v ktlint &> /dev/null; then
        # Format the hand-written test tree only; the generated output/ is
        # regenerated on every build and is exempt from lint by design.
        ktlint --format "kotlin/src/**/*.kt"
    else
        echo "[warn] ktlint not installed; skipping format step. (brew install ktlint)"
    fi
else
    echo "[warn] Java/gradlew not available; skipping Kotlin checks."
fi

# Java
if [ -x java/gradlew ] && command -v java &> /dev/null; then
    (cd java && ./gradlew compileTestJava)
    if command -v google-java-format &> /dev/null; then
        # Format the hand-written test tree only; the generated output/ is
        # regenerated on every build and is exempt from formatting by design.
        JAVA_SOURCES=$(find java/src -name '*.java')
        if [ -n "$JAVA_SOURCES" ]; then
            echo "$JAVA_SOURCES" | xargs google-java-format --replace
        fi
    else
        echo "[warn] google-java-format not installed; skipping format step. (brew install google-java-format)"
    fi
else
    echo "[warn] Java/gradlew not available; skipping Java checks."
fi

# Go
if command -v go &> /dev/null; then
    (cd go && go build ./...)
    (cd go && go vet ./...)
    # gofmt the hand-written test tree only; the generated output/ is regenerated
    # on every build and is emitted gofmt-clean by construction.
    GO_TEST_SOURCES=$(find go/tests -name '*.go' 2>/dev/null || true)
    if [ -n "$GO_TEST_SOURCES" ]; then
        gofmt -w go/tests
    fi
    if command -v golangci-lint &> /dev/null; then
        (cd go && golangci-lint run ./...)
    else
        echo "[warn] golangci-lint not installed; skipping lint step. (brew install golangci-lint)"
    fi
else
    echo "[warn] Go not on PATH; skipping Go checks."
fi