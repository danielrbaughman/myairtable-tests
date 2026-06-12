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