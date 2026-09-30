# Contributing to WaveFlow

WaveFlow is a Stellar/Soroban bounty escrow system: maintainers fund programs, contributors register Stellar wallets, merged PRs trigger gateway attestations that pay rewards on-chain. This guide explains how to pick up work, what conventions to follow, and how payouts flow through the Drips Wave program.

> WaveFlow is **Stellar-native** — the escrow, attestations, and payouts all happen on-chain. Per PRD non-goal NG6, WaveFlow does **not** depend on the Drips API. WaveFlow borrows the Wave *mechanics* (points to token payout on merge) but anchors them on Soroban. See [docs/PRD.md](docs/PRD.md#non-goals).

## Before you start

Read these in order before opening a PR:

1. [docs/PRD.md](docs/PRD.md) — what WaveFlow is, personas, acceptance criteria, non-goals.
2. [docs/ROADMAP.md](docs/ROADMAP.md) — phased delivery plan and production-readiness gates PR-1 through PR-8.
3. [docs/core-loop.md](docs/core-loop.md) — the merge to gateway to escrow to wallet path.
4. [docs/security-checklist.md](docs/security-checklist.md) — required hardening before contributing gateway or contract code.
5. [AGENTS.md](AGENTS.md) — quick command reference and high-value file map.

## Stellar Wave alignment

WaveFlow participates in the [Drips Wave](https://www.drips.network/wave) program. Maintainers assign complexity with these labels:

| Label | Meaning |
|------|---------|
| `Stellar Wave` | Issue is part of the Wave program and eligible for points. |
| `complexity:low` | Scaffold / docs / config — typically 1 point. |
| `complexity:medium` | Feature in an existing crate — typically 2-3 points. |
| `complexity:high` | New contract / new external integration / security-sensitive — typically 4-5 points. |

Points convert to XLM via the program's configured `points_to_token` ratio at the time the merge attestation is recorded. WaveFlow itself pays on-chain via the escrow contract; the Drips Wave layer is a coordinating ledger that mirrors the same merge events.

To earn, a contributor must:

1. Be assigned to an open `Stellar Wave`-labeled issue (comment `/assign` or `I'll take this`).
2. Open a PR that closes the issue (`Closes #N` in the PR body).
3. Have a Stellar wallet registered in the program (Phase 2 self-serve, Phase 0/1 maintainer-registered).
4. Wait for the merge — the gateway attestation fires within ~30 seconds of the GitHub webhook.

Payout lands on the contributor's registered wallet within one Soroban ledger close (~5 seconds). Failed attestations retry per the operational requirements in [PRD Section 8](docs/PRD.md#8-operational-requirements).

## Issue label prefixes

Issue titles should start with one of these prefixes so the gateway can route the merge event:

| Prefix | Scope | Example |
|--------|-------|---------|
| `[contracts]` | Soroban escrow / contract admin | `[contracts] add milestone cap to fund()` |
| `[gateway]` | GitHub webhook ingestion, HMAC, attestation | `[gateway] retry on transient Soroban RPC 5xx` |
| `[api]` | REST endpoints, auth, query paths | `[api] GET /programs/:id/payouts` |
| `[shared]` | Cross-crate types, errors, config | `[shared] unify ProgramId serialization` |
| `[infra]` | CI, docker, migrations, deploy | `[infra] pin Postgres 16 in compose` |
| `[documentation]` | Markdown, diagrams, runbooks | `[documentation] add CONTRIBUTING.md` |
| `[security]` | Threat model, key rotation, audits | `[security] rotate GATEWAY_SECRET_KEY runbook` |
| `[test]` | Coverage gaps, fuzz seeds, integration fixtures | `[test] HMAC invalid-signature corpus` |

Combined with the `Stellar Wave` and `complexity:*` labels, the prefix tells maintainers and the gateway what surface area the PR touches. Gateway routing rules live in [docs/gateway/](docs/gateway/).

## Development setup

```bash
git checkout main && git pull
git checkout -b feature/your-change
cp .env.example .env
docker-compose up -d
cargo build --workspace
cargo test --workspace
```

Required local services:

- Postgres 16+ (via docker-compose)
- Soroban CLI (only if you touch `contracts/` — see [docs/contracts/](docs/contracts/))

## Workspace layout

| Crate / path | Role |
|--------------|------|
| `contracts/waveflow-escrow` | Soroban escrow contract: `initialize`, `create_program`, `fund`, `register_contributor`, `record_merge`, events. |
| `crates/gateway` | GitHub webhook ingestion, HMAC verification, attestation payload builder. |
| `crates/api` | REST API for programs, contributors, payout history. |
| `crates/shared` | Config loader, shared types (`ProgramId`, `ContributorId`, `PointDelta`), error enum. |
| `migrations/` | Postgres schema, applied via `sqlx migrate`. |
| `docs/` | PRD, roadmap, security checklist, per-crate guides. |
| `fixtures/` | Sample webhook payloads and contract call frames for tests. |

## Pull requests

- Branch from `main`, open PRs against `main`. Long-lived feature branches are discouraged — break work into issues that fit one sprint.
- Title prefix must match the issue prefix (e.g. `[gateway] retry on transient Soroban RPC 5xx`).
- Body must include `Closes #N` so the gateway can correlate the merge with the Wave issue.
- Run `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --workspace` locally before pushing. CI runs the same three and fails the PR on any warning.
- For `contracts/` changes, also run `cargo test -p waveflow-escrow` and attach the contract test trace to the PR description.
- Keep PRs under ~600 lines of diff where possible. If an issue legitimately needs more, split it into a stack (PR A, PR B, PR C) and link them in the bodies.
- One issue per PR. If a PR resolves multiple issues, list them as `Closes #N, Closes #M` and explain in the body why they cannot be split.

### PR template

```markdown
## What
<one-paragraph summary>

## Why
<which PRD/roadmap section this addresses; which production-readiness gate>

## How
<bullet list of the changes, in execution order>

## Verification
- [ ] `cargo fmt --all -- --check` clean
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean
- [ ] `cargo test --workspace` green
- [ ] (contracts only) `cargo test -p waveflow-escrow` trace attached

Closes #N
```

## Security

WaveFlow handles real value transfer on a public ledger. Before contributing gateway or contract code:

1. Read [docs/security-checklist.md](docs/security-checklist.md) end to end.
2. Never commit real webhook secrets or Stellar secret keys. The `.env.example` lists all required vars; copy-then-fill locally.
3. Treat every webhook payload as untrusted until HMAC verification completes.
4. Idempotency key is `(program_id, pr_number)`. If you change the key shape, also update the gateway migration and the contract's `record_merge` boundary check.
5. Secret rotation paths are in [docs/operations/](docs/operations/). Open a `[security]` issue if you spot a missing rotation runbook.

## Maintainer runbook

Maintainers assigning `Stellar Wave` to a new issue:

1. Pick the complexity tier — `low` (1pt), `medium` (2-3pt), `high` (4-5pt). When in doubt, pick the lower tier; the gateway can re-rank on merge if a maintainer comment raises it.
2. Apply both labels — `Stellar Wave` and `complexity:*` — at issue creation. The gateway keys off both; missing either causes the merge attestation to be dropped into a `needs_review` bucket.
3. Assign a contributor only after they've confirmed they have a registered Stellar wallet in the target program (Phase 0/1: maintainer-registered via API; Phase 2: self-serve OAuth).
4. On merge, watch the gateway log for the attestation event. The contract emits `MergeRecorded(program_id, pr_number, contributor_id, points)` — verify the contributor wallet and points match the issue labels before considering the bounty closed.

## Getting help

- Open an issue with the `[documentation]` prefix for clarifications on this guide.
- Open an issue with the `[gateway]` prefix if a merge attestation looks missing or mis-routed.
- For security-sensitive reports, see [docs/security-checklist.md](docs/security-checklist.md) for the private disclosure path — do not open a public issue.
