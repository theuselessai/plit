#!/usr/bin/env bash
# Assemble config.yaml from config.d/ domain folders
#
# Structure:
#   config.d/base.yaml              → top-level config (admin, global)
#   config.d/listeners/*.yaml       → one file per listener (each is a bind)
#   config.d/backends/<provider>/    → one subfolder per LLM provider
#     _provider.yaml                → shared config (host, path, auth, TLS)
#     <model>.yaml                  → one file per model (just "model: <name>")
#   config.d/rules/*.yaml           → CEL authorization rules (merged into backends)
#   config.d/mcp_servers/*.yaml     → MCP server targets (injected into MCP listener)
#   config.d/jwt/jwks.json          → JWT public key (referenced by llm listener)
#
# Only providers with matching keys/<provider>.key are included.
# Output: config.yaml (atomic write via tmp + rename)

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_D="$SCRIPT_DIR/config.d"
KEYS_DIR="$SCRIPT_DIR/keys"
OUTPUT="$SCRIPT_DIR/config.yaml"
YQ="${YQ:-yq}"

# --- Step 1: Base config ---
merged=$($YQ eval '.' "$CONFIG_D/base.yaml")

# --- Step 2: Load authorization rules ---
# Collect all rules/*.yaml into a single array
rules="[]"
for f in "$CONFIG_D"/rules/*.yaml; do
    [ -f "$f" ] || continue
    rules=$(echo "$rules" | $YQ eval-all '
        select(fileIndex == 0) + select(fileIndex == 1)
    ' - "$f")
done

# --- Step 3: Add listeners as binds ---
for f in "$CONFIG_D"/listeners/*.yaml; do
    [ -f "$f" ] || continue
    merged=$(echo "$merged" | $YQ eval-all '
        select(fileIndex == 0).binds += [select(fileIndex == 1)]
        | select(fileIndex == 0)
    ' - "$f")
done

# --- Step 4: Inject backends into LLM listener routes ---
# Structure: config.d/backends/<provider>/_provider.yaml + <model>.yaml
# Each model file becomes a separate route, inheriting from _provider.yaml
# Only included if keys/<provider>.key exists
included=0
skipped=0
for provider_dir in "$CONFIG_D"/backends/*/; do
    [ -d "$provider_dir" ] || continue
    provider="$(basename "$provider_dir")"
    provider_file="$provider_dir/_provider.yaml"

    # Skip if no _provider.yaml
    if [ ! -f "$provider_file" ]; then
        echo "  ! $provider: missing _provider.yaml, skipped"
        continue
    fi

    # Skip if no matching key file
    if [ ! -d "$KEYS_DIR" ] || [ ! -f "$KEYS_DIR/${provider}.key" ]; then
        echo "  - $provider (no key, skipped)"
        skipped=$((skipped + 1))
        continue
    fi

    # Read provider config
    provider_config=$($YQ eval '.' "$provider_file")

    # Process each model file (skip _provider.yaml)
    for model_file in "$provider_dir"/*.yaml; do
        [ -f "$model_file" ] || continue
        [ "$(basename "$model_file")" = "_provider.yaml" ] && continue

        model_slug="$(basename "$model_file" .yaml)"
        model_name=$($YQ eval '.model' "$model_file")
        route_name="${provider}-${model_slug}"

        # Build the full route by merging provider + model
        route=$($YQ eval -n "{
            \"name\": \"${route_name}-route\",
            \"matches\": [{\"path\": {\"pathPrefix\": \"/${route_name}/\"}}],
            \"policies\": {
                \"authorization\": {\"rules\": []},
                \"backendAuth\": $(echo "$provider_config" | $YQ eval '.backendAuth' -o=json -),
                \"backendTLS\": $(echo "$provider_config" | $YQ eval '.backendTLS // {}' -o=json -)
            },
            \"backends\": [{
                \"ai\": {
                    \"provider\": $(echo "$provider_config" | $YQ eval '.provider' -o=json -),
                    \"name\": \"${route_name}\"
                    $(echo "$provider_config" | $YQ eval 'select(.hostOverride) | ", \"hostOverride\": \"" + .hostOverride + "\""' -)
                    $(echo "$provider_config" | $YQ eval 'select(.pathOverride) | ", \"pathOverride\": \"" + .pathOverride + "\""' -)
                }
            }]
        }")

        # Include backendTLS ONLY when the provider fragment declares it.
        # agentgateway treats the mere presence of the policy (even {}) as
        # "speak TLS to the upstream", which breaks plain-http backends
        # (e.g. a LAN Qwen box or local Ollama) with InvalidContentType.
        if [ "$(echo "$provider_config" | $YQ eval 'has("backendTLS")' -)" != "true" ]; then
            route=$(echo "$route" | $YQ eval 'del(.policies.backendTLS)' -)
        fi

        # Set the model override ONLY when the model file specifies a real
        # upstream model id. A missing/null model means pass-through:
        # agentgateway forwards the caller's requested model unchanged. Writing
        # a "null" (or empty) override would ship literal "null" upstream and
        # 404. (agentgateway provider.<x>.model is a hard outbound override.)
        if [ -n "$model_name" ] && [ "$model_name" != "null" ]; then
            route=$(echo "$route" | $YQ eval ".backends[0].ai.provider[][\"model\"] = \"${model_name}\"" -)
        fi

        # Inject authorization rules
        route=$(echo "$route" | $YQ eval-all '
            select(fileIndex == 0).policies.authorization.rules = select(fileIndex == 1)
            | select(fileIndex == 0)
        ' - <(echo "$rules"))

        # Append to LLM listener routes
        merged=$(echo "$merged" | $YQ eval-all '
            (select(fileIndex == 0).binds[] | select(.listeners[].name == "llm")).listeners[0].routes += [select(fileIndex == 1)]
            | select(fileIndex == 0)
        ' - <(echo "$route"))

        echo "  + ${provider}/${model_slug} (model: ${model_name})"
        included=$((included + 1))
    done
done

# --- Step 5: Inject MCP server targets into MCP listener ---
mcp_count=0
for f in "$CONFIG_D"/mcp_servers/*.yaml; do
    [ -f "$f" ] || continue
    mcp_name="$(basename "$f" .yaml)"

    merged=$(echo "$merged" | $YQ eval-all '
        (select(fileIndex == 0).binds[] | select(.listeners[].name == "mcp")).listeners[0].routes[0].backends += [select(fileIndex == 1)]
        | select(fileIndex == 0)
    ' - "$f")

    echo "  + mcp: $mcp_name"
    mcp_count=$((mcp_count + 1))
done

# --- Step 6: Atomic write ---
echo "$merged" > "${OUTPUT}.tmp"
mv "${OUTPUT}.tmp" "$OUTPUT"
echo "Assembled config.yaml ($included backends, $skipped skipped, $mcp_count mcp servers)"
