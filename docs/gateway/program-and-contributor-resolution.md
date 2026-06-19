# Program and contributor resolution

Before chain submission the gateway resolves Postgres rows from the GitHub webhook payload.

## Program lookup

`resolve_program_id(pool, github_repo)` queries:

```sql
SELECT id, on_chain_program_id FROM programs
WHERE github_repo = $1 AND status = 'active'
```

`github_repo` comes from `repository.full_name` in the webhook JSON (for example `StellarRoute/WaveFlow`).

## Contributor lookup

`resolve_contributor_address(pool, program_uuid, github_username)` requires a prior admin registration:

```sql
SELECT stellar_address FROM contributors
WHERE program_id = $1 AND github_username = $2
```

Missing rows surface as `NotFound` with the username in the error text.

See `crates/gateway/src/attestation.rs`.
