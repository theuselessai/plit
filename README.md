# plit

<p align="center">
  <strong>Pipelit ecosystem CLI</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/plit"><img src="https://img.shields.io/crates/v/plit.svg?style=flat-square" alt="crates.io" /></a>
  <a href="https://github.com/theuselessai/plit/releases"><img src="https://img.shields.io/github/v/release/theuselessai/plit?label=release&style=flat-square" alt="Release" /></a>
  <a href="https://github.com/theuselessai/plit/actions/workflows/ci.yml"><img src="https://github.com/theuselessai/plit/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-Apache%202.0-blue.svg?style=flat-square" alt="License: Apache 2.0" /></a>
</p>

**Supported platforms:** Linux, macOS, Windows, Raspberry Pi, iSH (iOS).

---

## Install

```sh
curl -sSf https://raw.githubusercontent.com/theuselessai/plit/main/install.sh | sh
```

Or via Cargo:

```sh
cargo install plit
```

Pre-built static binaries are available for Linux (x86_64, aarch64, i686), macOS (Intel, Apple Silicon), and Windows.

---

## Quick start

```sh
# 1. Interactive setup: configure Pipelit, your LLM provider, and credentials
plit init

# 2. Start the gateway, Pipelit, and workers
plit start
```

That's it. You're running.

---

## Commands

### `plit init`

Interactive wizard. Walks you through:

- Connecting to your Pipelit instance
- Configuring an LLM provider
- Creating initial credentials

On macOS and Windows, `plit init` detects the platform and switches to Docker mode automatically — pulling and running the [plit Docker image](https://github.com/orgs/theuselessai/packages/container/package/plit).

Run this once before anything else.

### `plit start` / `plit stop`

Start and stop the full stack: gateway, Pipelit, and background workers. On Linux, managed via honcho. On macOS/Windows, managed via Docker.

```sh
plit start
plit stop
```

### `plit chat`

Interactive REPL for chatting with your configured agent.

```sh
plit chat my_credential --chat-id session-1
```

### `plit send`

One-shot message. Pipe-friendly.

```sh
plit send my_credential --chat-id session-1 --text "Hello"
echo "summarize this" | plit send my_credential --chat-id session-1
```

Output is plain text on a TTY, JSON when piped. Pass `--json` to force JSON anywhere.

### `plit listen`

Stream incoming messages as JSONL over WebSocket. Useful for scripting and log tailing.

```sh
plit listen my_credential --chat-id session-1
plit listen my_credential --chat-id session-1 | jq .
```

### `plit credentials`

Manage gateway credentials.

```sh
plit credentials list
plit credentials create --name my-bot
plit credentials activate <id>
plit credentials deactivate <id>
```

### `plit health`

Check gateway and backend health.

```sh
plit health
```

### `plit uninstall`

Remove configuration and stop running services.

```sh
plit uninstall
```

---

## Environment variables

| Variable | Description |
|---|---|
| `GATEWAY_URL` | Gateway base URL (e.g. `http://localhost:8080`) |
| `GATEWAY_TOKEN` | Token for standard API access |
| `GATEWAY_ADMIN_TOKEN` | Token for admin operations (credentials, health) |

These override values from the config file written by `plit init`.

---

## Output modes

`plit` adapts its output to context:

- **TTY** — human-readable, formatted
- **Piped** — JSON automatically
- **`--json`** — force JSON regardless of context

---

## Architecture

`plit` is a thin CLI wrapper around `plit-gw`. Installing `plit` gives you both the command-line interface and the gateway server in a single `cargo install`. The gateway handles protocol adapters, message routing, and backend connections. The CLI handles setup, process management, and user-facing interaction.

For gateway configuration, adapter setup, and backend protocol details, see the [plit-gw repository](https://github.com/theuselessai/plit-gw).

---

## Related

- [plit-gw](https://github.com/theuselessai/plit-gw) — the gateway crate
- [Pipelit](https://github.com/theuselessai/Pipelit) — the agent orchestration platform

---

## License

Apache 2.0. See [LICENSE](LICENSE).
