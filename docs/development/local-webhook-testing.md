# Local GitHub webhook testing

Simulate a merged PR without configuring GitHub.

## Payload

Use `fixtures/merged_pr.json` at repo root.

## Compute HMAC signature

The digest is HMAC-SHA256 of the **raw JSON body** using `GITHUB_WEBHOOK_SECRET`, prefixed with `sha256=`:

```bash
SECRET="change-me-to-github-webhook-secret"
BODY=$(cat fixtures/merged_pr.json)
SIG="sha256=$(printf '%s' "$BODY" | openssl dgst -sha256 -hmac "$SECRET" | awk '{print $2}')"
```

## Send request

```bash
curl -X POST http://localhost:8080/webhooks/github \
  -H "Content-Type: application/json" \
  -H "X-GitHub-Event: pull_request" \
  -H "X-Hub-Signature-256: $SIG" \
  -d @fixtures/merged_pr.json
```

## Prerequisites

1. Gateway running (`cargo run -p waveflow-gateway`)
2. Program registered for repo in payload (`StellarRoute/WaveFlow`)
3. Contributor registered for GitHub username in payload
4. For on-chain payout: `ESCROW_CONTRACT_ID` and `GATEWAY_SECRET_KEY` set

## Verify

```bash
curl http://localhost:8081/api/v1/programs/<uuid>/payouts
```

See also [core-loop.md](../core-loop.md).
