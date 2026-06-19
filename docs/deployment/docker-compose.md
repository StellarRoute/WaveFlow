# Docker Compose

File: `docker-compose.yml`

## Purpose

Provides local Postgres for API and gateway development. Rust binaries run on the host via `cargo run`.

## Start

```bash
docker-compose up -d
```

## Connection string

From `.env.example`:

```
DATABASE_URL=postgres://waveflow:waveflow@localhost:5433/waveflow
```

Host port `5433` avoids conflicts with other local Postgres instances on 5432.

## Services

Typically a single `postgres` service with volume for data persistence. Check `docker-compose.yml` for the current image tag and credentials.

## With Rust services

Compose does not start gateway/API containers by default. Use Render Dockerfiles (`Dockerfile.gateway`, `Dockerfile.api`) as reference for production image builds.

## Stop and reset

```bash
docker-compose down
docker-compose down -v   # removes volume (fresh DB)
```
