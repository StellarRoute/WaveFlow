# Gateway webhooks

Service: `waveflow-gateway`

Default URL: `http://localhost:8080`

## `POST /webhooks/github`

Accepts GitHub `pull_request` webhook payloads when a PR merges.

### Required headers

| Header | Value |
|--------|-------|
| `Content-Type` | `application/json` |
| `X-GitHub-Event` | `pull_request` |
| `X-Hub-Signature-256` | `sha256=<hex digest>` |

HMAC uses `GITHUB_WEBHOOK_SECRET` over the raw request body. See [security/webhook-hmac.md](../security/webhook-hmac.md).

### Accepted events

Parsed by `parse_merge_event()` in `crates/gateway/src/webhook.rs`:

- `action` must be `closed`
- `pull_request.merged` must be `true`
- Base branch must equal repository `default_branch`

Other events return validation errors (ignored safely).

### Processing

1. Verify signature
2. Parse merge attestation (repo, username, PR number, points)
3. Resolve program and contributor in Postgres
4. Submit Soroban attestation (when `ESCROW_CONTRACT_ID` and keys configured)
5. Insert payout audit row

### Local testing

Use `fixtures/merged_pr.json` and [development/local-webhook-testing.md](../development/local-webhook-testing.md).

## `GET /health`

Gateway health check for load balancers and Render.
