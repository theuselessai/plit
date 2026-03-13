#!/bin/bash
set -e

CONFIG_FILE="/root/.config/plit/config.json"

if [ ! -f "$CONFIG_FILE" ]; then
    echo "First boot — configuring plit..."

    # Build args for plit init
    INIT_ARGS="--non-interactive --skip-install"
    INIT_ARGS="$INIT_ARGS --username ${ADMIN_USERNAME:-admin}"
    INIT_ARGS="$INIT_ARGS --password ${ADMIN_PASSWORD:?ADMIN_PASSWORD env var is required}"
    INIT_ARGS="$INIT_ARGS --llm-provider ${LLM_PROVIDER:?LLM_PROVIDER env var is required}"
    INIT_ARGS="$INIT_ARGS --llm-model ${LLM_MODEL:?LLM_MODEL env var is required}"

    [ -n "$LLM_API_KEY" ] && INIT_ARGS="$INIT_ARGS --api-key $LLM_API_KEY"
    [ -n "$LLM_BASE_URL" ] && INIT_ARGS="$INIT_ARGS --llm-base-url $LLM_BASE_URL"
    [ -n "$GATEWAY_PORT" ] && INIT_ARGS="$INIT_ARGS --gateway-port $GATEWAY_PORT"
    [ -n "$PIPELIT_PORT" ] && INIT_ARGS="$INIT_ARGS --pipelit-port $PIPELIT_PORT"

    INIT_ARGS="$INIT_ARGS --managed-dragonfly true"

    eval plit init $INIT_ARGS
fi

echo "Starting plit stack..."
exec plit start --foreground
