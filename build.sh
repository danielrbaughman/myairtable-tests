#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MYAIRTABLE_DIR="$SCRIPT_DIR/../myairtable"
AIRTABLE_DIR="$SCRIPT_DIR"

# Load environment variables
source "$AIRTABLE_DIR/.env"

# Which generator to run. By default this suite drives the PUBLISHED myairtable, so it
# exercises the exact code path ../airtable/generate.sh ships to production — a packaging
# defect (missing static/, a checkout-only path) fails here instead of shipping green.
# To test unreleased changes, set MYAIRTABLE_LOCAL=1 to run the sibling checkout instead.
# Output paths below are absolute, so no `cd` into the generator is needed.
MYAIRTABLE_VERSION="${MYAIRTABLE_VERSION:-0.1.0rc1}"
if [ -n "${MYAIRTABLE_LOCAL:-}" ]; then
    echo "Generating with local checkout: $MYAIRTABLE_DIR"
    MYAIRTABLE=(uv run --project "$MYAIRTABLE_DIR" myairtable)
else
    echo "Generating with published myairtable[cli]==$MYAIRTABLE_VERSION"
    MYAIRTABLE=(uvx --from "myairtable[cli]==$MYAIRTABLE_VERSION" myairtable)
fi

"${MYAIRTABLE[@]}" \
    --base-id "$AIRTABLE_BASE_ID" \
    --api-key "$AIRTABLE_API_KEY" \
    all \
    --meta-folder "$AIRTABLE_DIR" \
    --ts-folder "$AIRTABLE_DIR/typescript/output" \
    --js-folder "$AIRTABLE_DIR/javascript/output" \
    --py-folder "$AIRTABLE_DIR/python/output" \
    --rs-folder "$AIRTABLE_DIR/rust/output" \
    --swift-folder "$AIRTABLE_DIR/swift/output" \
    --kotlin-folder "$AIRTABLE_DIR/kotlin/output" \
    --java-folder "$AIRTABLE_DIR/java/output" \
    --go-folder "$AIRTABLE_DIR/go/output" \
    --csharp-folder "$AIRTABLE_DIR/csharp/output" \
    --cpp-folder "$AIRTABLE_DIR/cpp/output"
