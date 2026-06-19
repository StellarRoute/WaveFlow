# Public REST API

Service: `waveflow-api` (`crates/api/src/routes/mod.rs`)

Default base URL: `http://localhost:8081`

## `GET /health`

Returns service and database status:

```json
{
  "service": "waveflow-api",
  "database": true
}
```

HTTP 503 when database query fails.

## `GET /api/v1/programs`

List all programs ordered by `created_at` descending.

Returns array of `ProgramRecord` with fields: `id`, `on_chain_program_id`, `github_repo`, `maintainer_address`, `reward_per_point`, `escrow_balance`, `milestone_cap`, `milestone_spent`, `status`, `created_at`.

## `GET /api/v1/programs/:id`

Single program by UUID. HTTP 404 when not found.

## `GET /api/v1/programs/:id/payouts`

Payout history for a program. Each `PayoutRecord` includes `pr_number`, `github_username`, `stellar_address`, `points`, `amount`, optional `tx_hash`, and `created_at`.

## Authentication

Public read routes require no API key. Admin routes are documented in [admin-routes.md](admin-routes.md).
