# Contributing to WaveFlow

WaveFlow automates bounty escrow on Stellar/Soroban with GitHub merge-triggered payouts.

## Before you start

1. Read [docs/PRD.md](docs/PRD.md) for product scope.
2. Read [docs/ROADMAP.md](docs/ROADMAP.md) for phase plan.
3. Walk through [docs/core-loop.md](docs/core-loop.md) for the merge-to-payout path.
4. Review [AGENTS.md](AGENTS.md) for repository-specific implementation commands and guardrails.

WaveFlow is a Stellar-native implementation of Wave-style bounty mechanics. Per PRD NG6, it does not depend on or replace Drips APIs; it maps GitHub merge events to WaveFlow gateway attestations and Soroban escrow payouts.

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

## Pull requests

- Branch from `main`, open PRs against `main`.
- Run `cargo fmt`, `cargo clippy`, and `cargo test --workspace` before pushing.
- Use issue prefixes to keep scope clear:

| Prefix | Area |
|--------|------|
| `[contracts]` | Soroban escrow logic in `contracts/waveflow-escrow` |
| `[gateway]` | GitHub webhook ingestion and chain attestation in `crates/gateway` |
| `[api]` | REST API, admin routes, and read paths in `crates/api` |
| `[shared]` | Cross-crate types, config, and error handling in `crates/shared` |
| `[documentation]` | README, runbooks, and files under `docs/` |
| `[infra]` | Docker, Render, CI, migrations, and deployment support |

## Wave bounty labels

Label Wave-aligned issues with `Drips Wave` and a `complexity:*` tier. The current gateway points model is fixed at `1 point per merged PR` (see PRD OQ1 and `docs/gateway/payout-calculation.md`), so labels are used for maintainer planning and future point mapping rather than changing payout math today.

## Security

Never commit real webhook secrets or Stellar secret keys. See [docs/security-checklist.md](docs/security-checklist.md).
