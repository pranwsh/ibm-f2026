# Services

The stack is defined in `infra/docker-compose.yml`. Compose publishes host ports on loopback (`127.0.0.1`). Named volumes retain service data when containers are stopped.

| Service | Local address | Purpose |
|---|---|---|
| Postgres | `127.0.0.1:5432` | Application database |
| Kafka | `127.0.0.1:9092` | Single-node KRaft broker |
| ClickHouse | `127.0.0.1:8123`, `127.0.0.1:9000` | Analytics database |
| Vector | `127.0.0.1:8686` | Event pipeline configuration |
| API | `127.0.0.1:8000` | Python service scaffold |
| Frontend | `127.0.0.1:5173` | Vite frontend scaffold |
| Detector | `127.0.0.1:8081` | Rust service scaffold |

Start the standard stack with `make up`. For the lower-memory profile, use:

```sh
docker compose --env-file .env -f infra/docker-compose.yml -f infra/docker-compose.slim.yml up -d --build
```

The slim profile changes selected resource settings. Its values have not been confirmed against the referenced W3-4 requirements.

The API, frontend, detector, and Vector configuration are not yet a complete working pipeline. Healthchecks are configured in Compose, but some application healthchecks may not pass until the services are implemented.
