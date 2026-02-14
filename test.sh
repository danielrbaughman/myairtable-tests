#!/bin/bash
set -e

TEST_DIR="typescript/tests"

usage() {
    echo "Usage: ./test.sh [--all|--crud|--json|--filter|--runtime|--help]"
    echo ""
    echo "Options:"
    echo "  --all     Run all test suites (default)"
    echo "  --crud    Run CRUD test suites"
    echo "  --json    Run serializing tests"
    echo "  --filter  Run filter-by-formula tests"
    echo "  --runtime Run runtime-formulas tests"
    echo "  --help    Show this help message"
}

SUITE="${1:---all}"

case "$SUITE" in
    --help)
        usage
        exit 0
        ;;
    --all)
        TEST_CMD="yarn test"
        ;;
    --crud)
        TEST_CMD="npx vitest run $TEST_DIR/interface-crud-via-table.test.ts $TEST_DIR/model-crud-via-model.test.ts $TEST_DIR/model-crud-via-table.test.ts $TEST_DIR/record-crud-via-table.test.ts"
        ;;
    --json)
        TEST_CMD="npx vitest run $TEST_DIR/serializing.test.ts"
        ;;
    --filter)
        TEST_CMD="npx vitest run $TEST_DIR/filter-by-formula.test.ts"
        ;;
    --runtime)
        TEST_CMD="npx vitest run $TEST_DIR/runtime-formulas.test.ts"
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