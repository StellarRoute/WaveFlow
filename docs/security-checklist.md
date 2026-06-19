# WaveFlow security checklist (production)

## Webhook ingestion

- [ ] Set `WAVEFLOW_ENV=production` on deployed gateway services
- [ ] Use a random `GITHUB_WEBHOOK_SECRET` with at least 32 characters
- [ ] Confirm `GITHUB_WEBHOOK_SECRET` is not `change-me-to-github-webhook-secret`
- [ ] Rotate `GITHUB_WEBHOOK_SECRET` on a defined schedule
- [ ] Reject requests missing `X-Hub-Signature-256`
- [ ] Persist raw webhook payloads for forensic replay
- [ ] Enforce idempotency on `(program_id, pr_number)` in Postgres and on-chain

## Gateway oracle key

- [ ] Store `GATEWAY_SECRET_KEY` in a secrets manager (not plain env files in prod)
- [ ] Restrict gateway Soroban key balance alerts
- [ ] Monitor failed `record_merge` submissions

## API admin auth

- [ ] Configure non-default `API_ADMIN_KEYS`
- [ ] Terminate TLS at load balancer
- [ ] Rate-limit public endpoints

## Contract operations

- [ ] Verify escrow token contract address before funding
- [ ] Pause programs before `withdraw_remaining`
- [ ] Set milestone caps for grant budgets

## Observability

- [ ] Scrape `/metrics` from gateway and API
- [ ] Alert on `waveflow_gateway_webhook_failed_total` growth
- [ ] Log correlation via request IDs (TraceLayer on API)
