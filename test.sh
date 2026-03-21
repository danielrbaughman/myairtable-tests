#!/bin/bash
set -e

usage() {
    echo "Usage: ./test.sh <ts|js|py> [--all|--crud|--json|--filter|--runtime|--help]"
    echo ""
    echo "Arguments:"
    echo "  ts        Run TypeScript tests"
    echo "  js        Run JavaScript tests"
    echo "  py        Run Python tests"
    echo ""
    echo "Options:"
    echo "  --all     Run all test suites (default)"
    echo "  --crud    Run CRUD test suites"
    echo "  --json    Run serializing tests"
    echo "  --filter  Run filter-by-formula tests"
    echo "  --runtime Run runtime-formulas tests"
    echo "  --cache   Run caching tests"
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
    py)
        TEST_DIR="python/tests"
        ;;
    *)
        echo "Error: first argument must be 'ts', 'js', or 'py'"
        echo ""
        usage
        exit 1
        ;;
esac

shift
SUITE="${1:---all}"

if [ "$LANG_ARG" = "py" ]; then
    case "$SUITE" in
        --help)
            usage
            exit 0
            ;;
        --all)
            TEST_CMD="uv run pytest -x -v $TEST_DIR"
            ;;
        --crud)
            TEST_CMD="uv run pytest -x -v $TEST_DIR/test_dict_crud_via_table.py $TEST_DIR/test_model_crud_via_model.py $TEST_DIR/test_model_crud_via_table.py"
            ;;
        --json)
            TEST_CMD="uv run pytest -x -v $TEST_DIR/test_serializing.py"
            ;;
        --filter)
            TEST_CMD="uv run pytest -x -v $TEST_DIR/test_filter_by_formula.py"
            ;;
        --runtime)
            TEST_CMD="uv run pytest -x -v $TEST_DIR/test_runtime_formulas.py"
            ;;
        --cache)
            TEST_CMD="uv run pytest -x -v $TEST_DIR/test_caching.py"
            ;;
        *)
            echo "Unknown option: $SUITE"
            echo ""
            usage
            exit 1
            ;;
    esac
else
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
        --cache)
            TEST_CMD="npx vitest run $TEST_DIR/caching.test.$EXT"
            ;;
        *)
            echo "Unknown option: $SUITE"
            echo ""
            usage
            exit 1
            ;;
    esac
fi

./build.sh

if [ "$LANG_ARG" = "py" ]; then
    uv sync
    uv run ruff check
    uv run ruff format
    $TEST_CMD
else
    if ! command -v nvm &> /dev/null; then
        export NVM_DIR="$HOME/.nvm"
        [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
    fi

    nvm use

    yarn install
    yarn lint
    yarn format
    $TEST_CMD
fi
