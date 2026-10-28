# Signal pipeline

Local development scaffold for the event pipeline.

## Quick start

Requirements: Docker Engine with Compose v2. Nix and direnv are optional developer tools.

```sh
cp .env.example .env
make up
```

See [Development](docs/development.md) for tool setup and commands, [Services](docs/services.md) for the local stack and slim profile, and [Kafka topics](docs/kafka-topics.md) for topic configuration. The API, frontend, and detector are currently scaffolds; the full event pipeline is not implemented.

## Repository layout

- `backend/`: Python API scaffold
- `detector/`: Rust detector scaffold
- `frontend/`: Vite frontend scaffold
- `pipeline/vector/`: Vector configuration
- `content/rules/`: detection rules
- `fixtures/`: sample data
- `contracts/`: shared schemas
- `infra/`: Compose stack and infrastructure configuration
