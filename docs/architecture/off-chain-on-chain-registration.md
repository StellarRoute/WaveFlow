# Off-chain vs on-chain registration

WaveFlow maintains contributor and program records in Postgres **and** on the Soroban escrow contract. Both must agree before a merge payout succeeds.

## Admin API (Postgres)

`POST /admin/programs` and contributor registration routes write to `programs` and `contributors` tables. These rows supply:

- `github_repo` → `resolve_program_id()`
- `github_username` → `resolve_contributor_address()`
- `on_chain_program_id` for attestation building

## On-chain contract

`create_program` and `register_contributor` in `waveflow-escrow` store authoritative escrow state. Gateway attestation uses the numeric `on_chain_program_id` from Postgres, not the UUID primary key.

## Failure modes

- Active program missing in Postgres → `NotFound` before chain call
- Contributor not registered → `NotFound` with username in message
- On-chain program paused → contract returns `ProgramPaused` (error code 9)

Bootstrap operators should register in Postgres via admin API and mirror programs on Soroban during deployment.
