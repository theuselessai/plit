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
# Check both plit .env and pipelit .env for AGENTGATEWAY_DIR
AGW_DIR=""
PLIT_ENV="/root/.config/plit/.env"
PIPELIT_ENV="/root/.local/share/plit/pipelit/.env"
for envfile in "$PLIT_ENV" "$PIPELIT_ENV"; do
    if [ -f "$envfile" ] && [ -z "$AGW_DIR" ]; then
        AGW_DIR=$(grep "^AGENTGATEWAY_DIR=" "$envfile" | cut -d= -f2 | tr -d '"')
    fi
done

# Copy agentgateway env vars to Pipelit's .env if not already there
if [ -n "$AGW_DIR" ] && [ -f "$PIPELIT_ENV" ]; then
    for var in AGENTGATEWAY_ENABLED AGENTGATEWAY_URL AGENTGATEWAY_DIR JWT_PRIVATE_KEY; do
        if grep -q "^${var}=" "$PLIT_ENV" 2>/dev/null && ! grep -q "^${var}=" "$PIPELIT_ENV" 2>/dev/null; then
            grep "^${var}=" "$PLIT_ENV" >> "$PIPELIT_ENV"
        fi
    done
    # JWT_PRIVATE_KEY may be multiline — handle separately
    if grep -q "^JWT_PRIVATE_KEY=" "$PLIT_ENV" 2>/dev/null && ! grep -q "^JWT_PRIVATE_KEY=" "$PIPELIT_ENV" 2>/dev/null; then
        python3 -c "
import re
with open('$PLIT_ENV') as f: content = f.read()
m = re.search(r'(JWT_PRIVATE_KEY=\".*?\")', content, re.DOTALL)
if m:
    with open('$PIPELIT_ENV', 'a') as f: f.write('\n' + m.group(1) + '\n')
"
    fi
fi

if [ -n "$AGW_DIR" ] && [ -d "$AGW_DIR" ]; then
    echo "Starting agentgateway..."

    # Ensure binary is in place
    if [ ! -f "$AGW_DIR/bin/agentgateway" ]; then
        mkdir -p "$AGW_DIR/bin"
        cp /usr/local/bin/agentgateway "$AGW_DIR/bin/agentgateway"
    fi

    # Get encryption key for key decryption (check both .env files)
    FIELD_ENC_KEY=""
    for envfile in "$PIPELIT_ENV" "$PLIT_ENV"; do
        if [ -f "$envfile" ] && [ -z "$FIELD_ENC_KEY" ]; then
            FIELD_ENC_KEY=$(grep "^FIELD_ENCRYPTION_KEY=" "$envfile" | head -1 | cut -d= -f2 | tr -d '"')
        fi
    done

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
