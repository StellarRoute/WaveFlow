# Database schema

WaveFlow uses Postgres for off-chain metadata and payout audit. Migrations run on API/gateway startup.

## `programs`

| Column | Type | Notes |
|--------|------|-------|
| `id` | UUID | Primary key for API references |
| `on_chain_program_id` | BIGINT | Links to Soroban program |
| `github_repo` | TEXT | `owner/repo` slug |
| `maintainer_address` | TEXT | Stellar address |
| `reward_per_point` | BIGINT | Payout multiplier |
| `escrow_balance` | BIGINT | Cached on-chain balance (optional sync) |
| `milestone_cap` | BIGINT | Optional spending cap |
| `milestone_spent` | BIGINT | Running total toward cap |
| `status` | TEXT | `active` or `paused` |
| `created_at` | TIMESTAMPTZ | Row creation time |

## `contributors`

Maps `(program_id, github_username)` to `stellar_address`.

## `payouts`

Audit trail for each merge-triggered payout:

| Column | Purpose |
|--------|---------|
| `pr_number` | GitHub PR number |
| `github_username` | Contributor login |
| `points` | Points awarded (default 1 from webhook parser) |
| `amount` | Stellar stroops or token minor units |
| `tx_hash` | On-chain transaction hash when available |

## Idempotency

Gateway should reject duplicate `(program_id, pr_number)` payouts. Implementation details live in gateway attestation and SQL constraints.

## Connection

Set `DATABASE_URL` in `.env`. Docker Compose exposes Postgres on host port `5433`.
