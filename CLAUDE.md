# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with this repository.

## Project Overview

**plit** is the CLI and Docker image for the Pipelit ecosystem. It installs two binaries: `plit` (CLI) and `plit-gw` (gateway server). The Docker image bundles everything: plit, plit-gw, Pipelit backend, DragonflyDB, and a React frontend.

## Tools
- `gh` CLI is configured and working for GitHub operations (PRs, checks, merges, etc.)

## Architecture

```
plit (CLI)
├── plit init          → Interactive/non-interactive setup wizard
├── plit start/stop    → Manages full stack via honcho (gateway, Pipelit, workers, DragonflyDB)
├── plit local chat/send/listen → Permissionless gateway debug commands
└── plit credentials   → Manage gateway credentials

Docker Image (ghcr.io/theuselessai/plit)
├── Stage 1: Clone Pipelit (supports PIPELIT_REPO/PIPELIT_REF build args)
├── Stage 2: Build plit + plit-gw (Rust)
├── Stage 3: Build React frontend (Node)
└── Stage 4: Final image (python:3.12-slim + everything)
```

## Related Repos

| Repo | What | Default Branch |
|------|------|----------------|
| [plit](https://github.com/theuselessai/plit) | CLI + Docker image (this repo) | main |
| [plit-gw](https://github.com/theuselessai/plit-gw) | Gateway crate | main |
| [Pipelit](https://github.com/theuselessai/Pipelit) | Backend engine | **master** |

## Build & Test

```bash
cargo fmt --all -- --check    # Format check
cargo clippy -- -D warnings   # Lint
cargo test --all-features     # Unit tests
cargo build --release         # Release build
```

## Docker

```bash
# Build locally
docker build -t plit-e2e:local .

# Build with specific Pipelit branch
docker build --build-arg PIPELIT_REF=feat/my-branch -t plit-e2e:local .

# Run
docker run -d --name plit \
  -p 8080:8080 -p 8000:8000 \
  -e ADMIN_USERNAME=admin \
  -e ADMIN_PASSWORD=yourpassword \
  -e LLM_PROVIDER=anthropic \
  -e LLM_MODEL=claude-sonnet-4-20250514 \
  -e LLM_API_KEY=sk-... \
  plit-e2e:local
```

## Release Workflow (IMPORTANT — follow this process)

### Release Lifecycle

Releases follow a strict RC → stable promotion flow. **Never tag a stable version directly.**

```
v0.4.0-rc.1  →  smoke E2E (mock LLM)  →  GHCR :0.4.0-rc.1 + GitHub prerelease
v0.4.0-rc.2  →  smoke E2E (mock LLM)  →  GHCR :0.4.0-rc.2 + GitHub prerelease  (if fixes needed)
v0.4.0       →  full E2E (real key)    →  GHCR :0.4.0 + :latest + crates.io + GitHub stable release
```

### Step-by-Step Release Process

#### 1. Bump version

```bash
# Edit Cargo.toml: version = "0.4.0"
cargo update -p plit
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to 0.4.0"
git push origin main
```

#### 2. Tag RC

```bash
git tag v0.4.0-rc.1
git push origin v0.4.0-rc.1
```

This triggers the release workflow which will:
- Build Docker image
- Run smoke E2E (mock LLM, from Pipelit repo's `e2e/` scripts)
- Push to GHCR as `:0.4.0-rc.1`
- Create GitHub prerelease

#### 3. Validate RC

- Check the [Actions tab](https://github.com/theuselessai/plit/actions) — all jobs must be green
- Optionally pull and test locally: `docker pull ghcr.io/theuselessai/plit:0.4.0-rc.1`
- If issues found: fix, push to main, tag `v0.4.0-rc.2`

#### 4. Promote to stable

```bash
# Tag the SAME commit as the passing RC
git tag v0.4.0
git push origin v0.4.0
```

This triggers the release workflow which will:
- Build Docker image
- Run full E2E (real Anthropic API key via `LLM_API_KEY` org secret)
- Push to GHCR as `:0.4.0` and `:latest`
- Publish to crates.io (via `CARGO_REGISTRY_TOKEN` org secret)
- Create GitHub stable release

### Docker Tag Strategy

| Tag | What | Updated when |
|-----|------|-------------|
| `:X.Y.Z` | Immutable stable release | Stable tag pushed |
| `:X.Y.Z-rc.N` | Release candidate | RC tag pushed |
| `:latest` | Most recent stable | Stable tag pushed (never RC) |

### CI Secrets (org-level)

| Secret | Purpose |
|--------|---------|
| `LLM_API_KEY` | Real Anthropic API key for full E2E on stable releases |
| `CARGO_REGISTRY_TOKEN` | crates.io publish token (scope: `publish-update`) |

### E2E Test Suites

E2E scripts live in the **Pipelit** repo at `e2e/`. The release workflow checks them out.

| Suite | LLM | When | What it validates |
|-------|-----|------|-------------------|
| Smoke | Mock server | RC tags | Container boot, auth, user CRUD, API keys, gateway health, chat round-trip |
| Full | Real Anthropic | Stable tags | All smoke tests + tool use + sandbox execution |

### Anti-Patterns

- **NEVER** tag a stable release without a passing RC first
- **NEVER** rebuild the Docker image for stable — it should be the same code as the RC
- **NEVER** publish RC versions to crates.io
- **NEVER** point `:latest` at an RC

## Working Style

- Always create a new branch before implementing features
- Run `cargo fmt` and `cargo clippy -- -D warnings` before every push
- Pipelit's default branch is `master`, not `main`
