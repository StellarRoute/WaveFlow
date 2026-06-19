# WaveFlow documentation

## Product

| Doc | Description |
|-----|-------------|
| [PRD.md](PRD.md) | Product requirements |
| [ROADMAP.md](ROADMAP.md) | Implementation phases |
| [core-loop.md](core-loop.md) | Merge to payout walkthrough |

## Architecture

| Doc | Description |
|-----|-------------|
| [architecture/overview.md](architecture/overview.md) | System diagram and crates |
| [architecture/crate-boundaries.md](architecture/crate-boundaries.md) | Gateway vs API vs contract |

## API

| Doc | Description |
|-----|-------------|
| [api/rest-api.md](api/rest-api.md) | Public read endpoints |
| [api/admin-routes.md](api/admin-routes.md) | Program and contributor registration |
| [api/gateway-webhooks.md](api/gateway-webhooks.md) | GitHub webhook handler |

## Contracts

| Doc | Description |
|-----|-------------|
| [contracts/waveflow-escrow.md](contracts/waveflow-escrow.md) | Escrow contract overview |
| [contracts/storage-and-events.md](contracts/storage-and-events.md) | Storage keys and events |

## Operations

| Doc | Description |
|-----|-------------|
| [development/SETUP.md](development/SETUP.md) | Local install |
| [development/testing.md](development/testing.md) | cargo test guide |
| [development/local-webhook-testing.md](development/local-webhook-testing.md) | Simulate GitHub webhooks |
| [configuration/env-vars.md](configuration/env-vars.md) | Environment reference |
| [database/schema.md](database/schema.md) | Postgres tables |
| [deployment/render.md](deployment/render.md) | Render blueprint |
| [deployment/docker-compose.md](deployment/docker-compose.md) | Local Postgres |
| [security/webhook-hmac.md](security/webhook-hmac.md) | HMAC verification |
| [troubleshooting/common-issues.md](troubleshooting/common-issues.md) | Common problems |
| [security-checklist.md](security-checklist.md) | Pre-production checklist |
