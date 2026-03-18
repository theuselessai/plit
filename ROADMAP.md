# Roadmap

Milestones use **PROJECT** version from `VERSION` file. Each component (plit, plit-gw, plit-tui, Pipelit) has its own semver — the PROJECT version is the combined release.

Current: `PROJECT=0.4.3` | Next: `v0.5.0`

---

## v0.5.0 — Workflow Creation & API Client

Conversation-first workflow builder. Non-technical users describe what they want, agents build the workflow.

### Pipelit (backend)

| Issue | Title | Priority |
|-------|-------|----------|
| [#167](https://github.com/theuselessai/Pipelit/issues/167) | OpenAPI spec versioning + stability contract | P0 |
| [#163](https://github.com/theuselessai/Pipelit/issues/163) | Workflow DSL format spec + server-side parser | P0 |
| [#127](https://github.com/theuselessai/Pipelit/issues/127) | Skill to Workflow — distill skills into deterministic workflows | P1 |
| [#164](https://github.com/theuselessai/Pipelit/issues/164) | Scribe agent — requirements gathering via conversation | P1 |
| [#165](https://github.com/theuselessai/Pipelit/issues/165) | Architect + Gherkin agents — parallel topology and scenario generation | P1 |
| [#166](https://github.com/theuselessai/Pipelit/issues/166) | Builder agent — create workflows via API from verified topology | P1 |

### plit (CLI)

| Issue | Title | Priority |
|-------|-------|----------|
| [#17](https://github.com/theuselessai/plit/issues/17) | Auto-generate Rust API client crate from OpenAPI spec | P0 |
| [#18](https://github.com/theuselessai/plit/issues/18) | `plit api` subcommands — CLI wrapper over generated client | P0 |
| [#20](https://github.com/theuselessai/plit/issues/20) | DSL parser in Tela (JSX) for graph-boxes renderer | P1 |
| [#19](https://github.com/theuselessai/plit/issues/19) | Workflow graph visualization in plit-tui | P1 |

### plit-gw (gateway)

No work this milestone.

### Critical path

```
Pipelit #167 (OpenAPI versioning)
  → plit #17 (generate Rust client)
    → plit #18 (plit api subcommands)

Pipelit #163 (DSL spec)
  → plit #20 (JSX DSL parser)
    → plit #19 (graph TUI)
  → Pipelit #164 (Scribe)
  → Pipelit #165 (Architect + Gherkin)
    → Pipelit #166 (Builder) ← also needs plit #18
```

### Target VERSION on ship

```
PROJECT=0.5.0
PLIT=0.5.0
PLIT_GW=0.3.2     (unchanged)
PLIT_TUI=0.2.0
PIPELIT=0.4.0
```

---

## v0.6.0 — Protocol Adapters, Memory & Safety

### Pipelit

| Issue | Title |
|-------|-------|
| [#128](https://github.com/theuselessai/Pipelit/issues/128) | Human Confirmation — interrupt_before for manual approval |
| [#129](https://github.com/theuselessai/Pipelit/issues/129) | Meta Agent — observer and manager for agent behavior |
| [#131](https://github.com/theuselessai/Pipelit/issues/131) | Memory Tables — foundation for self-evolving agent |
| [#132](https://github.com/theuselessai/Pipelit/issues/132) | Memory Nodes — read/write agent knowledge |
| [#133](https://github.com/theuselessai/Pipelit/issues/133) | TOTP Verification Node |

### plit-gw

| Issue | Title |
|-------|-------|
| [#9](https://github.com/theuselessai/plit-gw/issues/9) | Discord Adapter |
| [#10](https://github.com/theuselessai/plit-gw/issues/10) | Slack Adapter |
| [#11](https://github.com/theuselessai/plit-gw/issues/11) | Email Adapter |
| [#60](https://github.com/theuselessai/plit-gw/issues/60) | Create theuselessai/registry |
| [#61](https://github.com/theuselessai/plit-gw/issues/61) | Create theuselessai/adapters |
| [#64](https://github.com/theuselessai/plit-gw/issues/64) | plit skills/workflows install |
| [#65](https://github.com/theuselessai/plit-gw/issues/65) | plit adapters install |
| [#66](https://github.com/theuselessai/plit-gw/issues/66) | Skills repo cleanup |
| [#34](https://github.com/theuselessai/plit-gw/issues/34) | Full OpenCode server mode integration |
| [#62](https://github.com/theuselessai/plit-gw/issues/62) | Create theuselessai/workflows — workflow template repository |

---

## Shipped

### v0.4.x (current)

Alpha + Beta: core security, sandboxed execution, multi-model gateway, Docker deployment, user management, RBAC, E2E CI.

### Version history

| PROJECT | PLIT | PLIT_GW | PLIT_TUI | PIPELIT | Date |
|---------|------|---------|----------|---------|------|
| 0.4.3 | 0.4.3 | 0.3.2 | 0.1.2 | 0.3.9 | 2026-03-16 |
| 0.4.0 | 0.4.0 | 0.3.0 | 0.1.0 | 0.3.9 | 2026-03-14 |
