#!/bin/bash
set -e

usage() {
    echo "Usage: ./test.sh <ts|js> [--all|--crud|--json|--filter|--runtime|--help]"
    echo ""
    echo "Arguments:"
    echo "  ts        Run TypeScript tests"
    echo "  js        Run JavaScript tests"
    echo ""
    echo "Options:"
    echo "  --all     Run all test suites (default)"
    echo "  --crud    Run CRUD test suites"
    echo "  --json    Run serializing tests"
    echo "  --filter  Run filter-by-formula tests"
    echo "  --runtime Run runtime-formulas tests"
    echo "  --help    Show this help message"
}

LANG_ARG="$1"

case "$LANG_ARG" in
    ts)
        TEST_DIR="typescript/tests"
        EXT="ts"
        ;;
    js)
        TEST_DIR="javascript/tests"
        EXT="js"
        ;;
    *)
        echo "Error: first argument must be 'ts' or 'js'"
        echo ""
        usage
        exit 1
        ;;
esac

shift
SUITE="${1:---all}"

case "$SUITE" in
    --help)
        usage
        exit 0
        ;;
    --all)
        TEST_CMD="npx vitest run $TEST_DIR"
        ;;
    --crud)
        TEST_CMD="npx vitest run $TEST_DIR/interface-crud-via-table.test.$EXT $TEST_DIR/model-crud-via-model.test.$EXT $TEST_DIR/model-crud-via-table.test.$EXT $TEST_DIR/record-crud-via-table.test.$EXT"
        ;;
    --json)
        TEST_CMD="npx vitest run $TEST_DIR/serializing.test.$EXT"
        ;;
    --filter)
        TEST_CMD="npx vitest run $TEST_DIR/filter-by-formula.test.$EXT"
        ;;
    --runtime)
        TEST_CMD="npx vitest run $TEST_DIR/runtime-formulas.test.$EXT"
        ;;
    *)
        echo "Unknown option: $SUITE"
        echo ""
        usage
        exit 1
        ;;
esac

./build.sh

# uv sync
# uv run ruff check
# uv run ty check
# uv run pytest
# uv run ruff format

if ! command -v nvm &> /dev/null; then
    export NVM_DIR="$HOME/.nvm"
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
fi

nvm use

yarn install
yarn lint
yarn format
$TEST_CMD
