# Prometheus metrics

Both gateway and API expose `/metrics` for Prometheus scraping.

## Gateway counters

| Metric | Meaning |
|--------|---------|
| `waveflow_gateway_webhook_total` | All webhook deliveries |
| `waveflow_gateway_webhook_rejected_total` | HMAC failures |
| `waveflow_gateway_webhook_ignored_total` | Validation skips |
| `waveflow_gateway_webhook_failed_total` | Processing errors |
| `waveflow_gateway_payout_success_total` | Successful payouts |

## API counters

| Metric | Meaning |
|--------|---------|
| `waveflow_api_health_checks_total` | Public health probe hits |

Install the recorder in each service `main.rs` and mount `/metrics` on the shared Axum router.

See `crates/gateway/src/routes.rs` and `crates/api/src/routes/mod.rs`.
