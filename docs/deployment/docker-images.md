# Docker images

WaveFlow ships separate multi-stage Dockerfiles for gateway and API binaries.

## Dockerfile.gateway

Builds `waveflow-gateway` from the workspace `Cargo.toml`, linking `crates/gateway` and shared crates.

## Dockerfile.api

Builds `waveflow-api` with the same workspace context but different binary target.

## Render wiring

`render.yaml` references these images for production services with `DATABASE_URL` and webhook secrets injected at deploy time.

Local `docker-compose.yml` may build one image tag for both services with different commands; check the compose file for the current dev shortcut.

See root `Cargo.toml` workspace members list.
