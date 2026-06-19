# Contributing to WaveFlow

WaveFlow automates bounty escrow on Stellar/Soroban with GitHub merge-triggered payouts. Contributions should stay aligned with the PRD, roadmap, and the merge-to-payout core loop.

## Before you start

1. Read [docs/PRD.md](docs/PRD.md) for product scope.
2. Read [docs/ROADMAP.md](docs/ROADMAP.md) for phase planning.
3. Walk through [docs/core-loop.md](docs/core-loop.md) for the merge-to-payout path.
4. Check [AGENTS.md](AGENTS.md) for repository-specific agent guidance.

Pick an open issue before opening a pull request. Prefer issues with a clear scope, acceptance criteria, and a complexity label. If the scope is unclear, ask for clarification on the issue before starting work.

## Development setup

```bash
git checkout main && git pull
git checkout -b feature/your-change
cp .env.example .env
docker-compose up -d
cargo build --workspace
cargo test --workspace
```

## Workspace layout

| Crate / path | Role |
|--------------|------|
| `contracts/waveflow-escrow` | Soroban escrow contract |
| `crates/gateway` | GitHub webhooks and chain attestation |
| `crates/api` | REST API for programs and payouts |
| `crates/shared` | Config, types, errors |
| `migrations/` | Postgres schema and migrations |
| `docs/` | Product, architecture, operations, and setup documentation |

## Issue label workflow

Issue titles use a prefix to identify the main area of the repository:

| Prefix | Area | Typical files |
|--------|------|---------------|
| `[contracts]` | Soroban escrow logic | `contracts/waveflow-escrow/` |
| `[gateway]` | GitHub webhook ingestion and chain attestation | `crates/gateway/` |
| `[api]` | REST API and admin/read paths | `crates/api/` |
| `[shared]` | Shared types, config, and errors | `crates/shared/` |
| `[database]` | Postgres schema and migrations | `migrations/` |
| `[infra]` | Docker, Render, deployment, and operational config | `Dockerfile.*`, `docker-compose.yml`, `render.yaml`, `scripts/` |
| `[documentation]` | README, runbooks, and guides | `README.md`, `docs/`, `AGENTS.md` |

Maintainers may also apply program and planning labels:

- `Stellar Wave` or `Drips Wave` marks work that maps to the Wave-style contributor program.
- `complexity:low`, `complexity:medium`, and `complexity:high` indicate the expected effort and point mapping.
- Feature labels such as `F1` through `F7` map issues to the PRD feature sections.
- Launch-gate labels such as `PR-1` through `PR-8` map issues to the production readiness gates in the roadmap.

When an issue is part of the Wave workflow, the maintainer-assigned complexity label is the source of truth for point mapping.

## Branch and pull request conventions

Use a short, descriptive branch name based on the issue scope:

```text
<area>-<short-description>
```

Examples:

- `gateway-verify-delivery-id`
- `api-program-payouts`
- `docs-maintainer-runbook`

Pull requests should:

- Branch from `main` and open PRs against `main`.
- Link the issue with `Fixes #<issue-number>` when the PR fully resolves it.
- Keep the change focused on one issue or one feature slice.
- Include the commands you ran under a validation or testing section.
- Update documentation when behavior, environment variables, APIs, or operations change.
- Avoid placeholder production paths, simulated transaction hashes, or hardcoded secrets.

## Stellar-native scope

WaveFlow is an independent Stellar/Soroban implementation of Wave-style bounty mechanics. The expected flow is:

1. Maintainers lock Stellar assets in Soroban escrow.
2. Contributors register Stellar wallets.
3. The gateway verifies GitHub merge events.
4. The contract pays rewards on-chain using the configured point-to-token ratio.

Keep contributions inside that model unless a maintainer explicitly changes the PRD.

## Local validation

Run the smallest command set that proves your change. Common commands from the repository root are:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

For focused work, use crate-specific checks such as:

```bash
cargo test -p waveflow-escrow record_merge_pays_contributor
cargo test -p waveflow-gateway verify_github_signature
```

Document any command you could not run and why.

## Security

Never commit real webhook secrets or Stellar secret keys. See [docs/security-checklist.md](docs/security-checklist.md).
