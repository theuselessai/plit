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

# Start agentgateway if configured
AGW_DIR=""
PIPELIT_ENV="/root/.local/share/plit/pipelit/.env"
if [ -f "$PIPELIT_ENV" ]; then
    AGW_DIR=$(grep "^AGENTGATEWAY_DIR=" "$PIPELIT_ENV" | cut -d= -f2 | tr -d '"')
fi

if [ -n "$AGW_DIR" ] && [ -d "$AGW_DIR" ]; then
    echo "Starting agentgateway..."

    # Ensure binary is in place
    if [ ! -f "$AGW_DIR/bin/agentgateway" ]; then
        mkdir -p "$AGW_DIR/bin"
        cp /usr/local/bin/agentgateway "$AGW_DIR/bin/agentgateway"
    fi

    # Get encryption key for key decryption
    FIELD_ENC_KEY=$(grep "^FIELD_ENCRYPTION_KEY=" "$PIPELIT_ENV" | head -1 | cut -d= -f2 | tr -d '"')

    # Start agentgateway in background
    (
        cd "$AGW_DIR" && \
        FIELD_ENCRYPTION_KEY="$FIELD_ENC_KEY" \
        PYTHON=/root/.local/share/plit/venv/bin/python3 \
        YQ=yq \
        ./start.sh > /tmp/agw.log 2>&1
    ) &

    # Wait for agentgateway to be ready (up to 10 seconds)
    for i in $(seq 1 10); do
        if curl -s -o /dev/null http://localhost:4000/ 2>/dev/null; then
            echo "agentgateway ready"
            break
        fi
        sleep 1
    done
fi

echo "Starting plit stack..."
exec plit start --foreground
