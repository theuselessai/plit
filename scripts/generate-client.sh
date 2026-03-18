#!/usr/bin/env bash
set -euo pipefail

# Generate the pipelit-client Rust crate from Pipelit's OpenAPI spec.
# Usage: ./scripts/generate-client.sh [path-to-openapi.json]
#
# If no path is given, fetches from http://localhost:8000/openapi.json

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CLIENT_DIR="$REPO_ROOT/pipelit-client"
SPEC="${1:-}"

if [ -z "$SPEC" ]; then
  SPEC="/tmp/pipelit-openapi.json"
  echo "Fetching OpenAPI spec from http://localhost:8000/openapi.json..."
  curl -sf http://localhost:8000/openapi.json > "$SPEC"
fi

echo "Spec: $SPEC"
echo "Output: $CLIENT_DIR"

# Preserve our Cargo.toml (don't let generator overwrite it)
cp "$CLIENT_DIR/Cargo.toml" /tmp/pipelit-client-cargo.toml.bak

# Generate
docker run --rm \
  --user "$(id -u):$(id -g)" \
  -v "$SPEC:/spec/openapi.json:ro" \
  -v "$CLIENT_DIR:/out" \
  openapitools/openapi-generator-cli:latest generate \
  -i /spec/openapi.json \
  -g rust \
  -o /out \
  --library reqwest \
  --additional-properties=packageName=pipelit-client,supportAsync=true

# Restore our Cargo.toml
mv /tmp/pipelit-client-cargo.toml.bak "$CLIENT_DIR/Cargo.toml"

# Clean up generator artifacts
rm -rf "$CLIENT_DIR/.openapi-generator" "$CLIENT_DIR/.travis.yml" \
  "$CLIENT_DIR/git_push.sh" "$CLIENT_DIR/.openapi-generator-ignore"

# Post-generation fixups:
# 1. Replace broken AnyOf types with serde_json::Value
find "$CLIENT_DIR/src" -name '*.rs' -exec \
  sed -i 's/Option<Option<Box<models::AnyOfLessThanGreaterThan>>>/Option<serde_json::Value>/g' {} +

# 2. Remove double_option serde attrs on those fields
find "$CLIENT_DIR/src" -name '*.rs' -exec \
  sed -i 's/, default, with = "::serde_with::rust::double_option"//g' {} +

# Format
cargo fmt --manifest-path "$CLIENT_DIR/Cargo.toml" 2>/dev/null || true

echo ""
echo "Done. Verify with: cargo build --manifest-path $CLIENT_DIR/Cargo.toml"
