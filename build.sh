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
    --api-key "$AIRTABLE_API_KEY" "$@" \
    --meta-folder "$AIRTABLE_DIR" \
    # --py-folder "$AIRTABLE_DIR/python" \
    # --js-folder "$AIRTABLE_DIR/javascript" \
    --ts-folder "$AIRTABLE_DIR/typescript"
