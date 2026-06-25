#!/bin/bash
set -e

usage() {
    echo "Usage: ./test.sh <all|ts|js|py|rs|swift|kotlin|java|go|cs> [--all|--crud|--json|--filter|--runtime|--help]"
    echo ""
    echo "Arguments:"
    echo "  all       Run tests for all languages"
    echo "  ts        Run TypeScript tests"
    echo "  js        Run JavaScript tests"
    echo "  py        Run Python tests"
    echo "  rs        Run Rust tests"
    echo "  swift     Run Swift tests"
    echo "  kotlin    Run Kotlin tests"
    echo "  java      Run Java tests"
    echo "  go        Run Go tests"
    echo "  cs        Run C# tests"
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
    all)
        SUITE="${2:---all}"
        for lang in ts js py rs swift kotlin java go cs; do
            echo ""
            echo "========================================="
            echo "  Running $lang tests ($SUITE)"
            echo "========================================="
            echo ""
            ./test.sh "$lang" "$SUITE"
        done
        exit 0
        ;;
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
    rs)
        RUST_DIR="rust"
        ;;
    swift)
        SWIFT_DIR="swift"
        ;;
    kotlin)
        KOTLIN_DIR="kotlin"
        ;;
    java)
        JAVA_DIR="java"
        ;;
    go)
        GO_DIR="go"
        ;;
    cs)
        CSHARP_DIR="csharp"
        ;;
    *)
        echo "Error: first argument must be 'all', 'ts', 'js', 'py', 'rs', 'swift', 'kotlin', 'java', 'go', or 'cs'"
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
            TEST_CMD="uv run pytest -x -v $TEST_DIR/test_dict_crud_via_table.py $TEST_DIR/test_model_crud_via_model.py $TEST_DIR/test_model_crud_via_table.py $TEST_DIR/test_pagination.py $TEST_DIR/test_error_paths.py"
            ;;
        --json)
            TEST_CMD="uv run pytest -x -v $TEST_DIR/test_serializing.py"
            ;;
        --filter)
            TEST_CMD="uv run pytest -x -v $TEST_DIR/test_filter_by_formula.py $TEST_DIR/test_formula_escaping.py"
            ;;
        --runtime)
            TEST_CMD="uv run pytest -x -v $TEST_DIR/test_runtime_formulas.py $TEST_DIR/test_runtime_formula_variety.py $TEST_DIR/test_primary_formula_runtime.py"
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
elif [ "$LANG_ARG" = "rs" ]; then
    case "$SUITE" in
        --help)
            usage
            exit 0
            ;;
        --all)
            TEST_CMD="cargo test --manifest-path Cargo.toml"
            ;;
        --crud)
            TEST_CMD="cargo test --test test_struct_crud_via_table --test test_orm_crud_via_table --test test_orm_crud_via_model --test test_pagination --test test_error_paths"
            ;;
        --json)
            TEST_CMD="cargo test --test test_serializing"
            ;;
        --filter)
            TEST_CMD="cargo test --test test_filter_by_formula --test test_formula_escaping"
            ;;
        --runtime)
            TEST_CMD="cargo test --test test_runtime_formulas --test test_runtime_formula_variety --test test_primary_formula_runtime"
            ;;
        --cache)
            TEST_CMD="cargo test --test test_caching"
            ;;
        *)
            echo "Unknown option: $SUITE"
            echo ""
            usage
            exit 1
            ;;
    esac
elif [ "$LANG_ARG" = "swift" ]; then
    # Swift Testing's --filter is a regex matched against suite/test names.
    # Swift test suites follow PascalCase "Test..." naming per the plan.
    case "$SUITE" in
        --help)
            usage
            exit 0
            ;;
        --all)
            TEST_CMD="swift test --package-path swift"
            ;;
        --crud)
            TEST_CMD="swift test --package-path swift --filter 'TestStructCrudViaTable|TestOrmCrudViaTable|TestOrmCrudViaModel|TestPagination|TestErrorPaths'"
            ;;
        --json)
            TEST_CMD="swift test --package-path swift --filter 'TestSerializing'"
            ;;
        --filter)
            TEST_CMD="swift test --package-path swift --filter 'TestFilterByFormula|TestFormulaEscaping'"
            ;;
        --runtime)
            TEST_CMD="swift test --package-path swift --filter 'TestRuntimeFormulas|TestRuntimeFormulaVariety|TestPrimaryFormulaRuntime'"
            ;;
        --cache)
            TEST_CMD="swift test --package-path swift --filter 'TestCaching'"
            ;;
        *)
            echo "Unknown option: $SUITE"
            echo ""
            usage
            exit 1
            ;;
    esac
elif [ "$LANG_ARG" = "kotlin" ]; then
    # Gradle's --tests filter matches fully-qualified class names; '*' wildcards
    # match on the simple class name. Kotlin test classes follow the PascalCase
    # "Test..." naming used by Swift.
    case "$SUITE" in
        --help)
            usage
            exit 0
            ;;
        --all)
            TEST_CMD="(cd kotlin && ./gradlew test)"
            ;;
        --crud)
            TEST_CMD="(cd kotlin && ./gradlew test --tests '*TestStructCrudViaTable' --tests '*TestOrmCrudViaTable' --tests '*TestOrmCrudViaModel' --tests '*TestPagination' --tests '*TestErrorPaths')"
            ;;
        --json)
            TEST_CMD="(cd kotlin && ./gradlew test --tests '*TestSerializing')"
            ;;
        --filter)
            TEST_CMD="(cd kotlin && ./gradlew test --tests '*TestFilterByFormula' --tests '*TestFormulaEscaping')"
            ;;
        --runtime)
            TEST_CMD="(cd kotlin && ./gradlew test --tests '*TestRuntimeFormulas' --tests '*TestRuntimeFormulaVariety' --tests '*TestPrimaryFormulaRuntime')"
            ;;
        --cache)
            TEST_CMD="(cd kotlin && ./gradlew test --tests '*TestCaching')"
            ;;
        *)
            echo "Unknown option: $SUITE"
            echo ""
            usage
            exit 1
            ;;
    esac
elif [ "$LANG_ARG" = "java" ]; then
    # Gradle's --tests filter matches fully-qualified class names; '*' wildcards
    # match on the simple class name. Java test classes follow the PascalCase
    # "Test..." naming used by Swift/Kotlin.
    case "$SUITE" in
        --help)
            usage
            exit 0
            ;;
        --all)
            TEST_CMD="(cd java && ./gradlew test)"
            ;;
        --crud)
            TEST_CMD="(cd java && ./gradlew test --tests '*TestStructCrudViaTable' --tests '*TestOrmCrudViaTable' --tests '*TestOrmCrudViaModel' --tests '*TestPagination' --tests '*TestErrorPaths')"
            ;;
        --json)
            TEST_CMD="(cd java && ./gradlew test --tests '*TestSerializing*')"
            ;;
        --filter)
            TEST_CMD="(cd java && ./gradlew test --tests '*TestFilterByFormula' --tests '*TestFormulaEscaping')"
            ;;
        --runtime)
            TEST_CMD="(cd java && ./gradlew test --tests '*TestRuntimeFormulas' --tests '*TestRuntimeFormulaVariety' --tests '*TestPrimaryFormulaRuntime')"
            ;;
        --cache)
            TEST_CMD="(cd java && ./gradlew test --tests '*TestCaching')"
            ;;
        *)
            echo "Unknown option: $SUITE"
            echo ""
            usage
            exit 1
            ;;
    esac
elif [ "$LANG_ARG" = "go" ]; then
    # `go test -run` takes a regex matched against test function names; Go test
    # functions follow the PascalCase "Test..." naming used by Swift/Kotlin/Java.
    case "$SUITE" in
        --help)
            usage
            exit 0
            ;;
        --all)
            TEST_CMD="(cd go && go test -v ./...)"
            ;;
        --crud)
            TEST_CMD="(cd go && go test -v -run 'TestStructCrudViaTable|TestOrmCrudViaTable|TestOrmCrudViaModel|TestPagination|TestErrorPaths' ./...)"
            ;;
        --json)
            TEST_CMD="(cd go && go test -v -run 'TestSerializing' ./...)"
            ;;
        --filter)
            TEST_CMD="(cd go && go test -v -run 'TestFilterByFormula|TestFormulaEscaping' ./...)"
            ;;
        --runtime)
            TEST_CMD="(cd go && go test -v -run 'TestRuntimeFormulas|TestRuntimeFormulaVariety|TestPrimaryFormulaRuntime' ./...)"
            ;;
        --cache)
            TEST_CMD="(cd go && go test -v -run 'TestCaching' ./...)"
            ;;
        *)
            echo "Unknown option: $SUITE"
            echo ""
            usage
            exit 1
            ;;
    esac
elif [ "$LANG_ARG" = "cs" ]; then
    # `dotnet test --filter` matches on FullyQualifiedName; '~' is a substring match.
    # C# test classes follow the PascalCase "Test..." naming used by Swift/Kotlin/Java.
    case "$SUITE" in
        --help)
            usage
            exit 0
            ;;
        --all)
            TEST_CMD="(cd csharp && dotnet test --nologo)"
            ;;
        --crud)
            TEST_CMD="(cd csharp && dotnet test --nologo --filter 'FullyQualifiedName~TestStructCrudViaTable|FullyQualifiedName~TestOrmCrudViaTable|FullyQualifiedName~TestOrmCrudViaModel|FullyQualifiedName~TestPagination|FullyQualifiedName~TestErrorPaths')"
            ;;
        --json)
            TEST_CMD="(cd csharp && dotnet test --nologo --filter 'FullyQualifiedName~TestSerializing')"
            ;;
        --filter)
            TEST_CMD="(cd csharp && dotnet test --nologo --filter 'FullyQualifiedName~TestFilterByFormula|FullyQualifiedName~TestFormulaEscaping')"
            ;;
        --runtime)
            TEST_CMD="(cd csharp && dotnet test --nologo --filter 'FullyQualifiedName~TestRuntimeFormulas|FullyQualifiedName~TestRuntimeFormulaVariety|FullyQualifiedName~TestPrimaryFormulaRuntime')"
            ;;
        --cache)
            TEST_CMD="(cd csharp && dotnet test --nologo --filter 'FullyQualifiedName~TestCaching')"
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
            TEST_CMD="npx vitest run $TEST_DIR/interface-crud-via-table.test.$EXT $TEST_DIR/model-crud-via-model.test.$EXT $TEST_DIR/model-crud-via-table.test.$EXT $TEST_DIR/record-crud-via-table.test.$EXT $TEST_DIR/pagination.test.$EXT $TEST_DIR/error-paths.test.$EXT"
            ;;
        --json)
            TEST_CMD="npx vitest run $TEST_DIR/serializing.test.$EXT"
            ;;
        --filter)
            TEST_CMD="npx vitest run $TEST_DIR/filter-by-formula.test.$EXT $TEST_DIR/formula-escaping.test.$EXT"
            ;;
        --runtime)
            TEST_CMD="npx vitest run $TEST_DIR/runtime-formulas.test.$EXT $TEST_DIR/runtime-formula-variety.test.$EXT $TEST_DIR/primary-formula-runtime.test.$EXT"
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
    $TEST_CMD
elif [ "$LANG_ARG" = "rs" ]; then
    $TEST_CMD
elif [ "$LANG_ARG" = "swift" ]; then
    eval "$TEST_CMD"
elif [ "$LANG_ARG" = "kotlin" ] || [ "$LANG_ARG" = "java" ]; then
    eval "$TEST_CMD"
elif [ "$LANG_ARG" = "go" ]; then
    eval "$TEST_CMD"
elif [ "$LANG_ARG" = "cs" ]; then
    export PATH="$PATH:$HOME/.dotnet/tools"
    eval "$TEST_CMD"
else
    if ! command -v nvm &> /dev/null; then
        export NVM_DIR="$HOME/.nvm"
        [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
    fi

    nvm use

    yarn install
    $TEST_CMD
fi

./checks.sh
