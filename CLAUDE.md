# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with this repository.

## Project Overview

**plit** is the CLI and Docker image for the Pipelit ecosystem. It installs two binaries: `plit` (CLI) and `plit-gw` (gateway server). The Docker image bundles everything: plit, plit-gw, Pipelit backend, DragonflyDB, and a React frontend.

## Roadmap & Versioning

**See `ROADMAP.md`** for the full project milestone plan.

Milestones use PROJECT version (from `VERSION` file), not component versions. The GitHub project board at https://github.com/orgs/theuselessai/projects/1 tracks project-level milestones. Each component repo (plit, plit-gw, Pipelit) has matching milestones named `vX.Y.0`.

Current: `PROJECT=0.4.3` | Next: **v0.5.0** (Workflow Creation & API Client)

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

## Versioning

Each component has an independent version. The Docker image is the **project release** — a tested combination of all three.

```
VERSION file (repo root):
  PROJECT=0.4.0        ← Docker tag, project release version
  PLIT=0.4.0           ← plit CLI version (Cargo.toml)
  PLIT_GW=0.3.2        ← plit-gw version (Cargo.toml)
  PIPELIT=0.3.9        ← Pipelit version (git tag on Pipelit repo)
```

- Bumping ANY component → update VERSION → new project release
- Docker `:latest` always points to the latest stable PROJECT version
- The release workflow reads VERSION to pin Pipelit to a specific tag (not master)

## Release Workflow (IMPORTANT — follow this process)

### Release Lifecycle

Releases follow a strict RC → stable promotion flow. **Never tag a stable version directly.**

```
v0.4.0-rc.1  →  full E2E (real key)    →  GHCR :0.4.0-rc.1 + GitHub prerelease
v0.4.0-rc.2  →  full E2E (real key)    →  GHCR :0.4.0-rc.2 + GitHub prerelease  (if fixes needed)
v0.4.0       →  no E2E (retag of RC)   →  GHCR :0.4.0 + :latest + crates.io + GitHub stable release
```

### Step-by-Step Release Process

#### 1. Update VERSION file

```bash
# Edit VERSION: bump PROJECT and any changed component versions
# Edit Cargo.toml if plit version changed
git add VERSION Cargo.toml Cargo.lock
git commit -m "chore: bump to 0.4.0"
git push origin main
```

#### 2. Tag RC

```bash
git tag v0.4.0-rc.1
git push origin v0.4.0-rc.1
```

This triggers the release workflow which will:
- Read VERSION for component pins
- Build Docker image with `PIPELIT_REF=v{PIPELIT}` (pinned tag, not master)
- Run full E2E (real Anthropic key via `LLM_API_KEY` org secret)
- Push to GHCR as `:0.4.0-rc.1`
- Create GitHub prerelease with component matrix

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
- Retag the RC Docker image as `:0.4.0` and `:latest` (no rebuild, no E2E — same code already validated)
- Build platform binaries for 6 targets (musl for Linux)
- Publish to crates.io (via `CARGO_REGISTRY_TOKEN` org secret)
- Create GitHub stable release with component matrix and binary downloads

### Docker Tag Strategy

| Tag | What | Updated when |
|-----|------|-------------|
| `:X.Y.Z` | Immutable stable release | Stable tag pushed |
| `:X.Y.Z-rc.N` | Release candidate | RC tag pushed |
| `:latest` | Most recent stable | Stable tag pushed (never RC) |

### CI Secrets (org-level)

| Secret | Purpose |
|--------|---------|
| `LLM_API_KEY` | Real Anthropic API key for full E2E on RC releases |
| `CARGO_REGISTRY_TOKEN` | crates.io publish token (scope: `publish-update`) |

### E2E Test Suites

E2E scripts live in the **Pipelit** repo at `e2e/`. The release workflow checks them out.

| Suite | LLM | When | What it validates |
|-------|-----|------|-------------------|
| Smoke | Mock server | PR (path-filtered) | Container boot, auth, user CRUD, API keys, gateway health, chat round-trip (mock) |
| Full | Real Anthropic | RC tags | All smoke tests + real LLM round-trip + tool use + sandbox execution |
| None | — | Stable tags | No E2E — stable is a retag of the validated RC |

### Anti-Patterns

- **NEVER** tag a stable release without a passing RC first
- **NEVER** rebuild the Docker image for stable — stable retags the RC image, same binary
- **NEVER** publish RC versions to crates.io
- **NEVER** point `:latest` at an RC

## Working Style

- Always create a new branch before implementing features
- Run `cargo fmt` and `cargo clippy -- -D warnings` before every push
- Pipelit's default branch is `master`, not `main`
