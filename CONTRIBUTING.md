# Contributing to WaveFlow

WaveFlow is a Stellar/Soroban bounty escrow system. Keep contributions small, auditable, and tied to the production readiness path in the PRD and roadmap.

## Before you start

- Pick an open issue whose scope matches the code or docs you plan to change.
- Check for an existing assignee, active pull request, or maintainer comment before starting.
- Prefer one issue per pull request. Split unrelated contract, gateway, API, and docs work.
- Do not introduce secrets, private data, paid services, or destructive actions into tests or examples.

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

WaveFlow follows Drips Wave-style contributor mechanics, but the product remains Stellar-native. PRD NG6 makes replacing or depending on StellarRoute DEX aggregation a non-goal, and the WaveFlow core loop must not depend on a Drips API. Treat Drips Wave as the program model, not as a runtime dependency.

For Wave-aligned issues:

1. Maintainers mark the issue with the Wave program label, such as `Drips Wave` or `Stellar Wave`.
2. Maintainers add one complexity label, such as `complexity:low`, `complexity:medium`, or `complexity:high`.
3. Gateway-related issues should reference the label-to-points mapping issue or PR when points affect payout calculation.
4. If no mapping issue is linked yet, include the proposed mapping in the PR description and keep the implementation configurable rather than hardcoded.
5. A merged PR should include enough context for the gateway to map repo, PR number, contributor, complexity tier, and points without relying on off-chain manual memory.

## Branch and pull request conventions

- Use a branch name that includes the issue number, for example `feature/38-contributing-guide`.
- Put `Closes #<issue>` or `Refs #<issue>` in the PR body.
- Keep the PR description focused on what changed, how it was validated, and any follow-up work.
- For code changes, run the relevant checks from `AGENTS.md` before requesting review.
- For documentation-only changes, mention that no Rust build was required.

## Quality bar

Contributions should preserve the PRD production target:

- fail closed on auth, signature, idempotency, and insufficient escrow failures;
- avoid placeholder production paths, simulated transaction hashes, and secret logging;
- update docs when behavior, environment variables, APIs, or operational steps change;
- include tests for changed contract, gateway, API, and shared logic where practical.

## Useful references

- [README](README.md)
- [Product Requirements](docs/PRD.md)
- [Implementation Roadmap](docs/ROADMAP.md)
- [Core loop walkthrough](docs/core-loop.md)
- [Agent guidance](AGENTS.md)
