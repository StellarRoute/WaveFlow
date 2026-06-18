# WaveFlow — Implementation Roadmap

Derived from [PRD.md](./PRD.md) (v0.2, Production target). Phases are sequential; each must meet exit criteria before the next begins. **Production-limited launch** requires all [PR-1 through PR-8](./PRD.md#7-production-readiness-criteria-v1-launch-gate) gates in the PRD.

---

## Production Readiness Gates (v1 launch)

| Gate | Phase | Primary owner | GitHub tracking |
|------|-------|---------------|-----------------|
| PR-1 Real Soroban RPC submission | 1 | gateway | #6 |
| PR-2 Webhook HMAC + delivery ID dedup | 2 | gateway | #9 |
| PR-3 Postgres audit trail complete | 2 | gateway, api | #8, #18 |
| PR-4 Health and readiness endpoints | 3 | gateway, api | #41 |
| PR-5 Prometheus metrics + alert thresholds | 3 | gateway, api | #21, #29 |
| PR-6 Security checklist executed | 3 | infra | #31, #32 |
| PR-7 Maintainer runbook published | 3 | documentation | #39 |
| PR-8 Contract deployed and verified | 1 | contracts, infra | #4, #40 |

Operational requirements (escrow alerts, attestation retry, key rotation, incident response) are defined in [PRD Section 8](./PRD.md#8-operational-requirements) and tracked in the issue backlog.

---

## Phase 0: Scaffold

| Field | Detail |
|-------|--------|
| **Objective** | Establish repo structure, toolchain, environment templates, and CI skeleton so all crates compile and tests run in a predictable workspace. |
| **Deliverables** | • Cargo workspace with `contracts/waveflow-escrow`, `crates/gateway`, `crates/api`, `crates/shared`<br>• Root `README.md`, `AGENTS.md`, `.env.example`, `docker-compose.yml`<br>• Postgres migration skeleton<br>• GitHub Actions: fmt, clippy, test<br>• `rust-toolchain.toml`, `.gitignore` |
| **Exit Criteria** | `cargo build --workspace` succeeds; `cargo test --workspace` runs (may be minimal); CI workflow file validates on push; docker-compose defines Postgres service. |
| **Estimated complexity** | **Low** |

---

## Phase 1: Core Loop

| Field | Detail |
|-------|--------|
| **Objective** | Prove the WaveFlow concept end-to-end: escrow funding, merge attestation, and single contributor payout in Soroban tests plus gateway webhook handler. |
| **Deliverables** | • Soroban contract: `initialize`, `create_program`, `fund`, `register_contributor`, `record_merge`, events<br>• Gateway: GitHub webhook route, HMAC verification, attestation payload builder<br>• Contract unit tests: fund → register → record_merge → balance transfer<br>• Gateway unit test: valid/invalid webhook parsing<br>• Integration doc in `docs/core-loop.md` |
| **Exit Criteria** | Contract test demonstrates payout math (`points × ratio`); gateway rejects bad HMAC and accepts merged PR payload; idempotency key `(program_id, pr_number)` defined in shared types. |
| **Estimated complexity** | **Medium** |

---

## Phase 2: Feature Complete

| Field | Detail |
|-------|--------|
| **Objective** | Implement remaining PRD core features: program admin, milestone caps, insufficient escrow handling, Postgres audit trail, and REST API for read/query paths. |
| **Deliverables** | • Contract: `pause`, `resume`, `set_ratio`, `withdraw_remaining`, milestone cap, `ProcessedPr` replay guard<br>• Postgres migrations: `programs`, `contributors`, `payouts`, `webhook_events`<br>• API: `/health`, `/api/v1/programs`, `/api/v1/programs/:id/payouts`<br>• Gateway persists audit rows; handles contributor-not-found and duplicate PR<br>• Error taxonomy in `crates/shared` |
| **Exit Criteria** | All PRD features F1–F7 have corresponding code paths; duplicate PR blocked on-chain and in DB; paused program rejects attestation; API returns payout history from DB. |
| **Estimated complexity** | **High** |

---

## Phase 3: Production Hardening

| Field | Detail |
|-------|--------|
| **Objective** | Make WaveFlow deployable and operable: authentication, rate limiting, structured observability, security headers, and deployment manifests. |
| **Deliverables** | • API key auth middleware for admin routes<br>• Tower rate limit + request ID + tracing middleware<br>• Prometheus metrics endpoint (`/metrics`)<br>• `Dockerfile` for gateway and API<br>• `render.yaml` (or equivalent) service definitions<br>• Security checklist doc: webhook replay window, secret rotation<br>• Startup dependency checks (DB, RPC URL present) |
| **Exit Criteria** | Unauthenticated admin calls return 401; metrics endpoint exposes request counters; Dockerfiles build multi-stage images; services bind `0.0.0.0:$PORT`; README documents production env vars; **PR-4 through PR-6 satisfied** per PRD launch gate table. |
| **Estimated complexity** | **Medium** |

---

## Summary Timeline

| Phase | Name | Complexity | Depends on | Primary output |
|-------|------|------------|------------|----------------|
| 0 | Scaffold | Low | — | Compilable monorepo + CI |
| 1 | Core Loop | Medium | Phase 0 | Escrow + merge → payout proven in tests |
| 2 | Feature Complete | High | Phase 1 | Full PRD feature set + API + DB |
| 3 | Production Hardening | Medium | Phase 2 | Auth, metrics, deploy configs |

**Suggested sequencing (calendar-agnostic):**

| Week block | Phase | Focus |
|------------|-------|-------|
| 1 | 0 | Repo bootstrap, CI green |
| 2–3 | 1 | Contract core + gateway webhook |
| 4–6 | 2 | Admin ops, milestones, API, Postgres |
| 7–8 | 3 | Auth, observability, deployment |

---

## Risk register (roadmap-level)

| Risk | Phase | Mitigation |
|------|-------|------------|
| Soroban SDK/RPC drift | 1+ | Pin SDK 21.x; document RPC version in README |
| GitHub webhook delivery gaps | 2 | Persist raw events; manual replay endpoint (admin) |
| Escrow depletion mid-program | 2 | Fail-closed contract + gateway alerting hook |
| Single gateway centralization | 3 | Document multisig oracle as v2; secure key management |
