# Crate boundaries

## `waveflow-shared`

**Path:** `crates/shared/`

Shared types (`ProgramRecord`, `PayoutRecord`, GitHub webhook payloads), errors (`WaveFlowError`), and `AppConfig` env loading. No HTTP or SQL logic.

## `waveflow-gateway`

**Path:** `crates/gateway/`

| Module | Responsibility |
|--------|----------------|
| `webhook.rs` | HMAC verification, merge event parsing |
| `attestation.rs` | Soroban transaction building and submission |
| `routes.rs` | Axum router for `/webhooks/github`, `/health` |

Gateway is the only service that receives GitHub webhooks and writes on-chain attestations.

## `waveflow-api`

**Path:** `crates/api/`

| Module | Responsibility |
|--------|----------------|
| `routes/mod.rs` | Public read routes: programs, payouts, health |
| `routes/admin.rs` | Admin POST routes for program/contributor setup |
| `middleware.rs` | API key validation for admin router |
| `startup.rs` | DB pool and migrations |

## `waveflow-escrow` (contract)

**Path:** `contracts/waveflow-escrow/`

Soroban contract: fund program, register contributors, record merge payouts. Tested in `src/test.rs` without external services.
