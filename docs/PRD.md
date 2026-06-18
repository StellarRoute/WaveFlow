# WaveFlow — Product Requirements Document

**Version:** 0.1  
**Status:** Draft  
**Repository:** [github.com/StellarRoute/WaveFlow](https://github.com/StellarRoute/WaveFlow)

---

## 1. Problem Statement

Open-source bounty programs on Stellar today lack a **ledger-native, automated payout pipeline** tied to verifiable contribution events (e.g. merged GitHub pull requests).

[Drips Wave](https://www.drips.network/wave) demonstrates a proven model: maintainers define reward pools, contributors earn points from merged PRs, and payouts flow automatically from a synced off-chain pipeline. That model is powerful but **not anchored on Stellar/Soroban**. Maintainers who want transparent, programmable, wallet-direct rewards must either:

- run manual treasury ops (slow, error-prone, unauditable), or
- rely on centralized payout rails disconnected from Stellar assets.

**WaveFlow** solves this by implementing the Wave mechanics **on-chain**: maintainers lock Stellar assets in a Soroban escrow contract; a verified GitHub merge event triggers an authorized gateway attestation; the contract computes reward from a configurable point-to-token ratio and pays the contributor's Stellar wallet immediately.

Core pain: **no trust-minimized, automated, Stellar-native bounty escrow with GitHub-verified triggers.**

---

## 2. Target Users

| Persona | Context | Primary need |
|---------|---------|--------------|
| **Program maintainer** | OSS repo owner or ecosystem grant operator funding Wave-style bounties | Lock funds, set point economics, link GitHub repo, monitor payouts without manual transfers |
| **Contributor** | Developer merging PRs into a registered repo | Register Stellar wallet, earn on merge, see payout status on-chain |
| **Gateway operator** | StellarRoute-style infra team running the attestation service | Secure webhook ingestion, idempotent attestations, observability |
| **Integrator / auditor** | Wallet, explorer, or compliance reviewer | Read-only API + on-chain events for program state and payout history |

**Assumed context:** Users are familiar with Stellar wallets (Freighter, etc.) and GitHub PR workflows. WaveFlow does not replace GitHub; it **reacts** to merge events.

---

## 3. Goals and Non-Goals

### Goals

| ID | Goal | Success signal |
|----|------|----------------|
| G1 | Ledger-native escrow for bounty pools (XLM or Soroban tokens) | Maintainer deposits; balance visible on-chain |
| G2 | GitHub merge → attestation → payout in one automated loop | Merged PR triggers payout without manual signer intervention |
| G3 | Configurable point-to-reward economics per program | Maintainer sets ratio; contract computes `points × ratio` |
| G4 | Idempotent, replay-safe attestations | Same PR cannot pay twice |
| G5 | Operable gateway + API mirroring StellarRoute backend patterns | Health checks, structured logs, Postgres audit trail |

### Non-Goals (v1)

| ID | Non-goal | Rationale |
|----|----------|-----------|
| NG1 | Full Drips/Wave product parity (leaderboards, social, multi-tenant SaaS UI) | Scope: escrow + payout engine only |
| N2 | On-chain GitHub light client / ZK proof of merge | Use authorized gateway attestation (oracle model) |
| NG3 | Streaming payments (e.g. continuous DCA) | Immediate discrete payouts per merge event |
| NG4 | Multi-chain or non-Stellar assets | Stellar/Soroban only |
| NG5 | Dispute resolution / clawback governance UI | Future phase; v1 supports maintainer pause only |
| NG6 | Replacing StellarRoute DEX aggregation | Separate product; may share infra patterns only |

---

## 4. Core Features

### F1 — Program creation and escrow funding

**Description:** Maintainer creates a bounty program linked to a GitHub repo (`owner/name`), sets point reward ratio, and deposits tokens into escrow.

| Acceptance criterion | Given | When | Then |
|---------------------|-------|------|------|
| AC-F1.1 | Maintainer has approved token allowance | `create_program` + `fund` called | Escrow balance increases; `ProgramCreated` event emitted |
| AC-F1.2 | Deposit below minimum | `fund` with zero amount | Transaction reverts with `InsufficientDeposit` |
| AC-F1.3 | Invalid repo string | `create_program` with malformed repo | Transaction reverts with `InvalidRepo` |

### F2 — Contributor registration

**Description:** Contributors map GitHub username to Stellar address for a program.

| Acceptance criterion | Given | When | Then |
|---------------------|-------|------|------|
| AC-F2.1 | Unregistered contributor | Maintainer or contributor calls `register_contributor` | Mapping stored; event emitted |
| AC-F2.2 | Duplicate registration same username | Second register with different address | Reverts with `ContributorAlreadyRegistered` |
| AC-F2.3 | Unregistered contributor at payout | Merge attestation arrives | Gateway rejects before chain tx; audit log records `contributor_not_found` |

### F3 — GitHub merge attestation (gateway)

**Description:** Gateway receives GitHub `pull_request` closed+merged webhooks, verifies HMAC, resolves contributor, submits on-chain attestation.

| Acceptance criterion | Given | When | Then |
|---------------------|-------|------|------|
| AC-F3.1 | Valid webhook for registered repo/program | PR merged to default branch | Gateway submits `record_merge`; payout executes |
| AC-F3.2 | Invalid HMAC | Webhook with bad signature | HTTP 401; no chain tx |
| AC-F3.3 | PR not merged | `closed` without merge | Ignored; no attestation |
| AC-F3.4 | Duplicate PR number | Replay webhook | Gateway or contract rejects; no double payout |

### F4 — Point-to-reward calculation and payout

**Description:** Contract computes payout as `points × reward_per_point` and transfers tokens to contributor wallet.

| Acceptance criterion | Given | When | Then |
|---------------------|-------|------|------|
| AC-F4.1 | Escrow sufficient; contributor registered | `record_merge(points=N)` | Contributor receives `N × ratio` tokens |
| AC-F4.2 | Escrow insufficient | Payout exceeds balance | Reverts `InsufficientEscrow`; gateway surfaces alert |
| AC-F4.3 | Program paused | Attestation submitted | Reverts `ProgramPaused` |

### F5 — Program administration

**Description:** Maintainer can pause/resume program, adjust ratio (with bounds), and withdraw unallocated escrow when paused.

| Acceptance criterion | Given | When | Then |
|---------------------|-------|------|------|
| AC-F5.1 | Active program | Maintainer calls `pause` | Further attestations rejected |
| AC-F5.2 | Paused program with remaining balance | Maintainer calls `withdraw_remaining` | Remaining escrow returned to maintainer |
| AC-F5.3 | Non-maintainer | Calls admin function | Reverts `Unauthorized` |

### F6 — Audit API and observability

**Description:** REST API exposes program status, payout history (from Postgres), and health endpoints.

| Acceptance criterion | Given | When | Then |
|---------------------|-------|------|------|
| AC-F6.1 | Gateway processed merge | GET `/api/v1/programs/:id/payouts` | Returns payout record with tx hash, PR number, amount |
| AC-F6.2 | Service healthy | GET `/health` | 200 with DB connectivity status |
| AC-F6.3 | Authenticated admin | POST with invalid API key | 401 |

### F7 — Milestone budgets (optional cap per epoch)

**Description:** Programs may define a milestone window (ledger sequence or timestamp) with max payout cap.

| Acceptance criterion | Given | When | Then |
|---------------------|-------|------|------|
| AC-F7.1 | Milestone cap reached | Additional merge attestation | Reverts `MilestoneCapExceeded` |
| AC-F7.2 | Milestone active | Payout within cap | Succeeds; milestone spent counter updated |

---

## 5. Technical Constraints

| Area | Constraint |
|------|------------|
| **Smart contracts** | Rust, Soroban SDK 21.x, `no_std`, WASM target |
| **Gateway + API** | Rust (Tokio, Axum), Postgres for audit/idempotency, optional Redis for rate limits |
| **Stellar network** | Testnet for development; mainnet-ready contract patterns (TTL extension, events) |
| **GitHub integration** | Webhooks (`pull_request` events), HMAC-SHA256 verification, repo slug `owner/name` |
| **Oracle model** | Single authorized gateway address per deployment; multisig upgrade path deferred |
| **Assets** | Native XLM and Soroban token contracts (SEP-41 style transfers) |
| **Deployment** | Docker Compose locally; Render-compatible services (`0.0.0.0:$PORT`) |
| **CI** | `cargo fmt`, `clippy -D warnings`, contract unit tests, gateway integration tests |
| **Secrets** | `DATABASE_URL`, `GITHUB_WEBHOOK_SECRET`, `SOROBAN_RPC_URL`, `GATEWAY_SECRET_KEY`, `API_ADMIN_KEYS` via env |
| **Idempotency** | Unique constraint on `(program_id, pr_number)` in Postgres; on-chain `ProcessedPr` storage |

**Environment assumptions:**

- Gateway has funded Stellar key for transaction submission fees.
- Maintainers interact via CLI/SDK or future UI; v1 scaffold includes API + contract client examples.
- GitHub delivers webhooks to a public HTTPS endpoint (ngrok acceptable in dev).

---

## 6. Open Questions

| # | Question | Impact | Default if unresolved |
|---|----------|--------|------------------------|
| OQ1 | **Points source:** fixed points per merge vs. label/size-based scoring? | Gateway logic + contract args | Fixed `1 point per merged PR` in v1 |
| OQ2 | **Who registers contributors:** maintainer-only vs. self-serve with GitHub OAuth? | API auth design | Maintainer registers via API; self-serve in v2 |
| OQ3 | **Gateway decentralization:** single operator vs. multisig oracle set? | Contract admin model | Single authorized gateway address |
| OQ4 | **Token allowlist:** any Soroban token vs. curated list? | Security / scam risk | Any token; maintainer bears risk |
| OQ5 | **Testnet vs. mainnet launch target?** | Deployment config | Testnet default in scaffold |
| OQ6 | **Relationship to Drips Wave:** complementary tooling vs. official integration? | Product positioning | Independent Stellar-native implementation; no Drips API dependency in v1 |
| OQ7 | **Partial payouts when escrow low:** fail vs. pay partial? | Contract behavior | Fail closed (revert) |
| OQ8 | **PR base branch filtering:** default branch only vs. configurable? | Webhook filter | Default branch only |

---

## Appendix — Reference flow

```mermaid
sequenceDiagram
    participant GH as GitHub
    participant GW as WaveFlow Gateway
    participant DB as Postgres
    participant SC as Soroban Escrow
    participant W as Contributor Wallet

    Note over SC: Maintainer funds escrow
    GH->>GW: pull_request merged webhook
    GW->>GW: Verify HMAC + idempotency
    GW->>DB: Check pr_number not processed
    GW->>SC: record_merge(program, contributor, pr, points)
    SC->>W: Token transfer (points × ratio)
    SC-->>GW: Tx success + events
    GW->>DB: Persist payout audit row
```
