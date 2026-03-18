# Stage 1: Clone Pipelit
FROM alpine/git:latest AS pipelit-source
ARG PIPELIT_REPO=https://github.com/theuselessai/Pipelit.git
ARG PIPELIT_REF=master
RUN git clone --depth 1 --branch "$PIPELIT_REF" "$PIPELIT_REPO" /pipelit

# Stage 2: Build plit + plit-gw
FROM rust:1-bookworm AS rust-builder
WORKDIR /build
COPY . .
RUN cargo build --release

# Stage 3: Build React frontend
FROM node:20-bookworm-slim AS node-builder
COPY --from=pipelit-source /pipelit/platform/frontend /frontend
WORKDIR /frontend
RUN npm ci && npm run build

# Stage 4: Final image
FROM python:3.12-slim-bookworm

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        gcc libffi-dev git ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

ENV HOME=/root
ENV FRONTEND_DIST_PATH=/app/frontend/dist

RUN mkdir -p /root/.config/plit /root/.local/share/plit

COPY --from=rust-builder /build/target/release/plit /usr/local/bin/plit
COPY --from=rust-builder /build/target/release/plit-gw /usr/local/bin/plit-gw

COPY --from=pipelit-source /pipelit /root/.local/share/plit/pipelit
COPY --from=node-builder /frontend/dist /app/frontend/dist

RUN python3 -m venv /root/.local/share/plit/venv && \
    /root/.local/share/plit/venv/bin/pip install --no-cache-dir \
        -r /root/.local/share/plit/pipelit/platform/requirements.txt && \
    /root/.local/share/plit/venv/bin/pip install --no-cache-dir honcho

RUN ARCH=$(uname -m) && \
    VERSION=$(curl -sS https://api.github.com/repos/dragonflydb/dragonfly/releases/latest | python3 -c "import json,sys;print(json.load(sys.stdin)['tag_name'])") && \
    curl -fSL "https://github.com/dragonflydb/dragonfly/releases/download/${VERSION}/dragonfly-${ARCH}.tar.gz" | tar xz -C /root/.config/plit && \
    mv "/root/.config/plit/dragonfly-${ARCH}" /root/.config/plit/dragonfly && \
    chmod +x /root/.config/plit/dragonfly

COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

EXPOSE 8080 8000

VOLUME ["/root/.local/share/plit", "/root/.config/pipelit/workspaces"]

ENTRYPOINT ["/entrypoint.sh"]
