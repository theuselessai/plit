#!/usr/bin/env bash
set -euo pipefail

# E2E tests for the generated pipelit-client via plit CLI.
# Requires: running Pipelit at $PIPELIT_URL with admin credentials.
#
# Usage: PIPELIT_URL=http://localhost:8000 PIPELIT_USER=admin PIPELIT_PASS=testpass123 ./e2e/api-client.sh [path-to-plit]

PLIT="${1:-./target/debug/plit}"
URL="${PIPELIT_URL:-http://localhost:8000}"
USER="${PIPELIT_USER:-admin}"
PASS="${PIPELIT_PASS:-}"
TOKEN="${PIPELIT_TOKEN:-}"

PASSED=0
FAILED=0

pass() { PASSED=$((PASSED + 1)); echo "  PASS: $1"; }
fail() { FAILED=$((FAILED + 1)); echo "  FAIL: $1"; }

echo "=== plit API client E2E tests ==="
echo "URL: $URL"
echo "Binary: $PLIT"
echo ""

# --- Auth tests ---
echo "--- Auth ---"

if [ -n "$TOKEN" ]; then
  $PLIT auth login --url "$URL" --token "$TOKEN" && pass "auth login --token" || fail "auth login --token"
elif [ -n "$PASS" ]; then
  $PLIT auth login --url "$URL" --username "$USER" --password "$PASS" && pass "auth login --password" || fail "auth login --password"
else
  echo "  SKIP: no PIPELIT_PASS or PIPELIT_TOKEN set"
fi

OUTPUT=$($PLIT auth status 2>&1)
echo "$OUTPUT" | grep -q "Logged in" && pass "auth status (logged in)" || fail "auth status: $OUTPUT"

# --- API endpoint tests (via curl using stored token) ---
echo ""
echo "--- API endpoints ---"

AUTH_JSON="${XDG_CONFIG_HOME:-$HOME/.config}/plit/auth.json"
if [ ! -f "$AUTH_JSON" ]; then
  echo "  SKIP: no auth.json found at $AUTH_JSON"
  exit 1
fi

API_TOKEN=$(python3 -c "import json; print(json.load(open('$AUTH_JSON'))['token'])")
API_URL=$(python3 -c "import json; print(json.load(open('$AUTH_JSON'))['pipelit_url'])")

api() {
  curl -sf -H "Authorization: Bearer $API_TOKEN" "$API_URL/api/v1/$1" 2>/dev/null
}

api_post() {
  curl -sf -X POST -H "Authorization: Bearer $API_TOKEN" -H "Content-Type: application/json" -d "$2" "$API_URL/api/v1/$1" 2>/dev/null
}

# GET /auth/me/
RESP=$(api "auth/me/")
echo "$RESP" | python3 -c "import json,sys; d=json.load(sys.stdin); assert d['username']" 2>/dev/null \
  && pass "GET /auth/me/" || fail "GET /auth/me/"

# GET /workflows/
RESP=$(api "workflows/")
echo "$RESP" | python3 -c "import json,sys; d=json.load(sys.stdin); assert 'items' in d" 2>/dev/null \
  && pass "GET /workflows/" || fail "GET /workflows/"

# GET /workflows/node-types/
RESP=$(api "workflows/node-types/")
echo "$RESP" | python3 -c "import json,sys; d=json.load(sys.stdin); assert len(d) > 0" 2>/dev/null \
  && pass "GET /workflows/node-types/" || fail "GET /workflows/node-types/"

# GET /credentials/
RESP=$(api "credentials/")
echo "$RESP" | python3 -c "import json,sys; d=json.load(sys.stdin); assert 'items' in d" 2>/dev/null \
  && pass "GET /credentials/" || fail "GET /credentials/"

# GET /executions/
RESP=$(api "executions/")
echo "$RESP" | python3 -c "import json,sys; d=json.load(sys.stdin); assert 'items' in d" 2>/dev/null \
  && pass "GET /executions/" || fail "GET /executions/"

# GET /health
RESP=$(curl -sf "$API_URL/health" 2>/dev/null)
echo "$RESP" | python3 -c "import json,sys; d=json.load(sys.stdin); assert d['status']" 2>/dev/null \
  && pass "GET /health" || fail "GET /health"

# --- CRUD test: create workflow, add node, add edge, validate, delete ---
echo ""
echo "--- CRUD ---"

SLUG="e2e-test-$$"

RESP=$(api_post "workflows/" "{\"name\":\"E2E Test\",\"slug\":\"$SLUG\"}")
echo "$RESP" | python3 -c "import json,sys; d=json.load(sys.stdin); assert d['slug'] == '$SLUG'" 2>/dev/null \
  && pass "POST /workflows/ (create)" || fail "POST /workflows/ (create): $RESP"

RESP=$(api "workflows/$SLUG/")
echo "$RESP" | python3 -c "import json,sys; d=json.load(sys.stdin); assert d['slug'] == '$SLUG'" 2>/dev/null \
  && pass "GET /workflows/$SLUG/" || fail "GET /workflows/$SLUG/"

NODE_RESP=$(api_post "workflows/$SLUG/nodes/" "{\"component_type\":\"trigger_manual\",\"label\":\"trigger\",\"position_x\":0,\"position_y\":0}")
NODE_ID=$(echo "$NODE_RESP" | python3 -c "import json,sys; print(json.load(sys.stdin)['id'])" 2>/dev/null)
[ -n "$NODE_ID" ] && pass "POST /nodes/ (create trigger)" || fail "POST /nodes/ (create trigger): $NODE_RESP"

NODE2_RESP=$(api_post "workflows/$SLUG/nodes/" "{\"component_type\":\"agent\",\"label\":\"agent\",\"position_x\":0,\"position_y\":100}")
NODE2_ID=$(echo "$NODE2_RESP" | python3 -c "import json,sys; print(json.load(sys.stdin)['id'])" 2>/dev/null)
[ -n "$NODE2_ID" ] && pass "POST /nodes/ (create agent)" || fail "POST /nodes/ (create agent): $NODE2_RESP"

if [ -n "$NODE_ID" ] && [ -n "$NODE2_ID" ]; then
  EDGE_RESP=$(api_post "workflows/$SLUG/edges/" "{\"source_node_id\":\"$NODE_ID\",\"target_node_id\":\"$NODE2_ID\",\"edge_type\":\"direct\"}")
  echo "$EDGE_RESP" | python3 -c "import json,sys; d=json.load(sys.stdin); assert d['id']" 2>/dev/null \
    && pass "POST /edges/ (create)" || fail "POST /edges/ (create): $EDGE_RESP"
fi

VALIDATE_RESP=$(curl -sf -X POST -H "Authorization: Bearer $API_TOKEN" "$API_URL/api/v1/workflows/$SLUG/validate/" 2>/dev/null)
[ $? -eq 0 ] && pass "POST /workflows/$SLUG/validate/" || fail "POST /workflows/$SLUG/validate/"

curl -sf -X DELETE -H "Authorization: Bearer $API_TOKEN" "$API_URL/api/v1/workflows/$SLUG/" > /dev/null 2>&1 \
  && pass "DELETE /workflows/$SLUG/" || fail "DELETE /workflows/$SLUG/"

# --- Auth logout ---
echo ""
echo "--- Cleanup ---"

$PLIT auth logout && pass "auth logout" || fail "auth logout"
OUTPUT=$($PLIT auth status 2>&1)
echo "$OUTPUT" | grep -q "Not logged in" && pass "auth status (logged out)" || fail "auth status after logout: $OUTPUT"

# --- Summary ---
echo ""
echo "=== Results: $PASSED passed, $FAILED failed ==="
[ "$FAILED" -eq 0 ] || exit 1
