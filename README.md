# Signal pipeline — local development

A small local stack for ingesting events and evaluating detection rules.

## Quickstart (about 15 minutes)

Requirements: Docker Engine + Compose v2, and Nix (or uv, Rust/Cargo, Bun and make).

```sh
cp .env.example .env
# Optional; provides pinned tools via the flake
direnv allow  # or: nix develop
make up
```

Compose builds the API, detector and web client, starts Postgres, Kafka (single KRaft node, no ZooKeeper), ClickHouse and Vector, then initializes topics. Open <http://127.0.0.1:5173> and <http://127.0.0.1:8000/docs>. Stop with `make down`; named volumes preserve data. `docker compose -f infra/docker-compose.yml -f infra/docker-compose.slim.yml --env-file .env up -d --build` applies the lower-memory slim profile.

## Layout

- `backend/`: Python API (uv)
- `detector/`: Rust detector
- `pipeline/vector/`: Vector configuration
- `frontend/`: Vite application
- `content/rules/`, `fixtures/`, `contracts/`: rules, sample inputs and schemas
- `infra/`: Compose stack and database initialization

Kafka topics are created idempotently by `kafka-init`, with auto-creation disabled: `raw-events` (source account key, 3 partitions, 7 days), `normalized-events` (source account, 3, 3 days), `detection-signals` (signal_id, 3, 7 days), and `quarantine` (source account, 1, 14 days). Values are original records with batch/source/received headers, NormalizedEvent JSON, DetectionSignal JSON, and original record plus error respectively.

Run `make test`, `make lint`, and `make fmt`. Local example passwords are in `.env.example`; do not use them outside development.
