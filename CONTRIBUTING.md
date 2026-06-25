# Contributing to WaveFlow

WaveFlow automates bounty escrow on Stellar/Soroban with GitHub merge-triggered payouts. Keep contributions small, auditable, and tied to the production readiness path in the PRD and roadmap.

## Before you start

- Read [docs/PRD.md](docs/PRD.md) for product scope.
- Read [docs/ROADMAP.md](docs/ROADMAP.md) for phase plan.
- Walk through [docs/core-loop.md](docs/core-loop.md) for the merge-to-payout path.
- Pick an open issue whose scope matches the code or docs you plan to change.
- Check for an existing assignee, active pull request, or maintainer comment before starting.
- Prefer one issue per pull request. Split unrelated contract, gateway, API, and docs work.

## Development setup

```bash
git checkout main && git pull
git checkout -b feature/your-change
cp .env.example .env
docker-compose up -d
cargo build --workspace
cargo test --workspace
```

Run the focused command for your change when possible, then run the wider workspace check before review if you touched shared behavior.

## Workspace layout

| Crate / path | Role |
|--------------|------|
| `contracts/waveflow-escrow` | Soroban escrow contract |
| `crates/gateway` | GitHub webhooks and chain attestation |
| `crates/api` | REST API for programs and payouts |
| `crates/shared` | Config, types, errors |
| `docs/` | PRD, roadmap, runbooks, and walkthroughs |

## Issue labels and title prefixes

Use issue title prefixes to make the affected area clear:

| Prefix | Area |
|--------|------|
| `[contract]` | Soroban escrow code in `contracts/waveflow-escrow` |
| `[gateway]` | GitHub webhook ingestion, HMAC checks, attestation, metrics, and chain submission in `crates/gateway` |
| `[api]` | REST routes, admin auth, health/readiness, and read models in `crates/api` |
| `[shared]` | Shared errors, types, config, and idempotency keys in `crates/shared` |
| `[infra]` | Docker, Render, migrations, deployment scripts, and environment templates |
| `[docs]` | README, PRD, roadmap, runbooks, agent notes, and walkthroughs |
| `[tests]` | Focused test-only changes or test coverage for an existing feature |
| `[security]` | Auth, replay protection, secret handling, rate limits, and fail-closed behavior |

Maintainers may also add status labels such as `documentation`, `good first issue`, or `help wanted` when a task is ready for contributors.

## Wave bounty label workflow

WaveFlow follows Drips Wave-style contributor mechanics, but the product remains Stellar-native. PRD NG6 keeps WaveFlow focused on its own Stellar/Soroban escrow and payout loop; the gateway should not depend on a Drips API for payout execution. Treat Drips Wave as the program model, not as a runtime dependency.

For Wave-aligned issues:

1. Maintainers mark the issue with the Wave program label, such as `Drips Wave` or `Stellar Wave`.
2. Maintainers add one complexity label, such as `complexity:low`, `complexity:medium`, or `complexity:high`.
3. Gateway-related issues should reference the label-to-points mapping issue or PR when points affect payout calculation.
4. If no mapping issue is linked yet, include the proposed mapping in the PR description and keep the implementation configurable rather than hardcoded.
5. A merged PR should include enough context for the gateway to map repo, PR number, contributor, complexity tier, and points without relying on off-chain manual memory.

## Pull requests

- Branch from `main`, open PRs against `main`.
- Use a branch name that includes the issue number, for example `feature/38-contributing-guide`.
- Put `Closes #<issue>` or `Refs #<issue>` in the PR body.
- Keep the PR description focused on what changed, how it was validated, and any follow-up work.
- For code changes, run `cargo fmt`, `cargo clippy`, and `cargo test --workspace` before pushing.
- For documentation-only changes, mention that no Rust build was required.

## Quality bar

Contributions should preserve the PRD production target:

- fail closed on auth, signature, idempotency, and insufficient escrow failures;
- avoid placeholder production paths, simulated transaction hashes, and secret logging;
- update docs when behavior, environment variables, APIs, or operational steps change;
- include tests for changed contract, gateway, API, and shared logic where practical.

## Security

Never commit real webhook secrets or Stellar secret keys. See [docs/security-checklist.md](docs/security-checklist.md).

Do not introduce secrets, private data, paid services, or destructive actions into tests or examples.

## Useful references

- [README](README.md)
- [Product Requirements](docs/PRD.md)
- [Implementation Roadmap](docs/ROADMAP.md)
- [Core loop walkthrough](docs/core-loop.md)
- [Agent guidance](AGENTS.md)
