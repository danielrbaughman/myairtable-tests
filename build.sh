#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MYAIRTABLE_DIR="$SCRIPT_DIR/../myairtable"
AIRTABLE_DIR="$SCRIPT_DIR"

# Load environment variables
source "$AIRTABLE_DIR/.env"

cd "$MYAIRTABLE_DIR"
uv run main.py \
    --base-id "$AIRTABLE_BASE_ID" \
    --api-key "$AIRTABLE_API_KEY" \
    all \
    --meta-folder "$AIRTABLE_DIR" \
    --ts-folder "$AIRTABLE_DIR/typescript/output" \
    --js-folder "$AIRTABLE_DIR/javascript/output" \
    --py-folder "$AIRTABLE_DIR/python/output" \
    --rs-folder "$AIRTABLE_DIR/rust/output" \
    --swift-folder "$AIRTABLE_DIR/swift/output"
