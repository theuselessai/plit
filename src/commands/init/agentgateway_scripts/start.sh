#!/usr/bin/env bash
# Start agentgateway:
#   1. Decrypt + load API keys from keys/ folder
#   2. Assemble config.yaml from config.d/ fragments
#   3. Start agentgateway
#
# Key file convention:
#   keys/venice.key  → exports VENICE_API_KEY=<contents>
#   keys/anthropic.key → exports ANTHROPIC_API_KEY=<contents>
#
# Config fragment convention:
#   config.d/00-base.yaml      → top-level config (admin, etc.)
#   config.d/01-mcp.yaml       → MCP listener (full bind)
#   config.d/02-llm-base.yaml  → LLM listener skeleton (JWT, empty routes)
#   config.d/10-*.yaml         → LLM route fragments (one per backend)

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
KEYS_DIR="$SCRIPT_DIR/keys"
YQ="${YQ:-yq}"

# Find Python with cryptography (prefer Pipelit venv, fall back to system)
if [ -z "${PYTHON:-}" ]; then
  for candidate in \
    "${PIPELIT_DIR:+$PIPELIT_DIR/.venv/bin/python3}" \
    "$HOME/.local/share/plit/venv/bin/python3" \
    "python3"; do
    [ -n "$candidate" ] && [ -x "$candidate" ] && PYTHON="$candidate" && break
  done
fi
PYTHON="${PYTHON:-python3}"

# --- Step 1: Decrypt and load API keys ---
if [ -d "$KEYS_DIR" ] && ls "$KEYS_DIR"/*.key 1>/dev/null 2>&1; then
  eval "$("${PYTHON:-python3}" "$SCRIPT_DIR/decrypt_keys.py" "$KEYS_DIR")"
  echo "Loaded keys from $KEYS_DIR"
fi

# --- Step 2: Assemble config from fragments ---
"$SCRIPT_DIR/assemble-config.sh"

# --- Step 3: Assemble final config ---
# Convert YAML to JSON (workaround for serde_yaml untagged enum bug with -f).
# Then resolve env: references to inline values since -f doesn't support env.
$YQ eval -o=json '.' "$SCRIPT_DIR/config.yaml" > "$SCRIPT_DIR/config.tmp.json"

# Replace {"env":"VAR_NAME"} with the actual env var value as a plain string
"${PYTHON}" -c "
import json, os, sys
with open('$SCRIPT_DIR/config.tmp.json') as f:
    data = json.load(f)

def resolve_env(obj):
    if isinstance(obj, dict):
        if set(obj.keys()) == {'env'} and isinstance(obj['env'], str):
            return os.environ.get(obj['env'], '')
        return {k: resolve_env(v) for k, v in obj.items()}
    elif isinstance(obj, list):
        return [resolve_env(v) for v in obj]
    return obj

data = resolve_env(data)
with open('$SCRIPT_DIR/config.json', 'w') as f:
    json.dump(data, f)
"
rm -f "$SCRIPT_DIR/config.tmp.json"

# --- Step 4: Start agentgateway ---
exec "$SCRIPT_DIR/bin/agentgateway" -f "$SCRIPT_DIR/config.json" "$@"
