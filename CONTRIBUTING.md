# Contributing to WaveFlow

WaveFlow automates bounty escrow on Stellar/Soroban with GitHub merge-triggered payouts.

## Before you start

1. Read [docs/PRD.md](docs/PRD.md) for product scope.
2. Read [docs/ROADMAP.md](docs/ROADMAP.md) for phase plan.
3. Walk through [docs/core-loop.md](docs/core-loop.md) for the merge-to-payout path.

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
- Keep PRs focused on one issue and include `Closes #<issue>` when the work fully satisfies the acceptance criteria.

## Issue prefixes and labels

Use bracketed prefixes in issue titles so contributors can quickly match work to the right crate, document, or operational surface:

| Prefix | Scope |
|--------|-------|
| `[contracts]` | Soroban escrow logic under `contracts/waveflow-escrow` |
| `[gateway]` | GitHub webhook ingestion, HMAC verification, and chain attestation in `crates/gateway` |
| `[api]` | REST read/admin routes, middleware, and API state in `crates/api` |
| `[shared]` | Shared config, error, and type definitions in `crates/shared` |
| `[documentation]` | README, PRD, roadmap, runbooks, and contributor docs |
| `[infra]` | Docker, Render, migrations, CI, and deployment support |

Wave-aligned issues should carry:

- `Drips Wave` for work intended to participate in Wave-style bounty accounting.
- `complexity:low`, `complexity:medium`, or `complexity:high` so maintainers can map the issue to the gateway point schedule.
- Any area label that helps routing, such as `contracts`, `gateway`, `api`, `documentation`, `security`, or `infra`.

When maintainers open or update the gateway point-mapping issue, link it from Wave bounty issues whose complexity label determines payout. If no mapping issue exists yet, document the expected point value directly in the issue body.

## Wave program alignment

WaveFlow is Stellar-native by design. It borrows the Wave contribution pattern of labelled issues, merged PRs, points, and payouts, but PRD non-goal NG6 keeps it independent from any Drips API dependency. Contributors should implement behavior against WaveFlow's Soroban escrow, gateway, API, and Postgres audit trail rather than assuming an external Wave service will calculate or submit payouts.

## Security

Never commit real webhook secrets or Stellar secret keys. See [docs/security-checklist.md](docs/security-checklist.md).
