# GitHub webhook HMAC verification

Implementation: `crates/gateway/src/webhook.rs` function `verify_github_signature()`.

## Algorithm

1. Read header `X-Hub-Signature-256` (must start with `sha256=`).
2. Decode hex digest after prefix.
3. Compute HMAC-SHA256 of raw request body with `GITHUB_WEBHOOK_SECRET`.
4. Compare using constant-time equality (`subtle` crate).

## Failure responses

| Condition | Error |
|-----------|-------|
| Missing `sha256=` prefix | Webhook error |
| Invalid hex | Webhook error |
| Digest mismatch | Unauthorized |

## Secret rotation

1. Generate new secret in GitHub repo webhook settings.
2. Update `GITHUB_WEBHOOK_SECRET` on gateway service.
3. Redeploy gateway before removing old secret in GitHub.

## Testing

Unit tests in `webhook.rs` cover valid signatures and tampered bodies. Use [local-webhook-testing.md](../development/local-webhook-testing.md) for end-to-end checks.
