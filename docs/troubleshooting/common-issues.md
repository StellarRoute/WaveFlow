# Troubleshooting

## API health shows `"database": false`

- Confirm Postgres is running: `docker-compose ps`
- Check `DATABASE_URL` matches compose credentials and port `5433`
- Review API logs for migration errors on startup

## Webhook returns 401 Unauthorized

- Verify `X-Hub-Signature-256` matches HMAC of **raw body** (not re-serialized JSON)
- Confirm `GITHUB_WEBHOOK_SECRET` matches the value used to compute signature

## Webhook ignored (validation error)

Common causes from `parse_merge_event()`:

- PR closed but not merged
- Merged to non-default branch
- Event action is not `closed`

## Payout not recorded

1. Program exists for `repository.full_name` in payload
2. Contributor registered for PR author login
3. Gateway has valid `ESCROW_CONTRACT_ID` and signing key for on-chain path
4. Escrow has sufficient balance

## Admin route returns 401

Send header `x-api-key` with a value from `API_ADMIN_KEYS` (comma-separated list in env).

## Contract deploy failures

Ensure Soroban CLI matches network in `SOROBAN_RPC_URL` and `NETWORK_PASSPHRASE`. See `scripts/deploy-contract.sh`.
