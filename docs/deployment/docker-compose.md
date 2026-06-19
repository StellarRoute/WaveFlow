# Docker Compose

File: `docker-compose.yml`

## Purpose

Provides a local WaveFlow stack with Postgres, the GitHub webhook gateway, and the REST API.

## Start

```bash
docker-compose up -d
```

The gateway is exposed at `http://localhost:8080`, and the API is exposed at `http://localhost:8081`.

Health checks:

```bash
curl http://localhost:8080/health
curl http://localhost:8081/health
```

## Connection string

From `.env.example`:

```
DATABASE_URL=postgres://waveflow:waveflow@localhost:5433/waveflow
```

Host port `5433` avoids conflicts with other local Postgres instances on 5432.

## Services

| Service | Purpose | Host port |
|---------|---------|-----------|
| `postgres` | Local Postgres database with persisted volume | `5433` |
| `waveflow-gateway` | GitHub webhook gateway | `8080` |
| `waveflow-api` | REST API | `8081` |

The Rust services load `.env`, override `DATABASE_URL` for the compose network, wait for a healthy Postgres container, and run SQLx migrations during startup.

## Host-based Rust services

For faster edit-compile cycles, start only Postgres and run the services on the host:

```bash
docker-compose up -d postgres
cargo run -p waveflow-gateway
cargo run -p waveflow-api
```

## Stop and reset

```bash
docker-compose down
docker-compose down -v   # removes volume (fresh DB)
```
