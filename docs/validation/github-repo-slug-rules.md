# GitHub repo slug validation

Repo strings are validated in Rust shared types and again on Soroban.

## Off-chain (`validate_github_repo`)

In `crates/shared/src/types.rs`:

- Non-empty, max 100 characters
- Must contain exactly one `/` separating owner and repo
- Owner and repo segments are non-empty

Admin `create_program` calls this before Postgres insert.

## On-chain (`validate_repo`)

Contract `storage.rs` enforces similar rules with Soroban `Symbol` length limits. Mismatches between admin input and contract rejection surface as `InvalidRepo` (code 4).

Keep admin registrations aligned with GitHub `repository.full_name` format (`Org/Repo`).

See `contracts/waveflow-escrow/src/test.rs::invalid_repo_is_rejected`.
