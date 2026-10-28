# Requirements review

Status of the fresh scaffold against the requested requirements. Compose configuration parses, but some acceptance criteria are only partially met or not implemented yet.

| Requirement | Status | Notes |
|---|---|---|
| Required repo layout | **Present, minimal** | The requested directories exist. Several contain only a README or placeholder. |
| Postgres 18 | **Present** | `postgres:18-alpine`, with a named volume, loopback port, and healthcheck. |
| Kafka 4.x, single KRaft node, no ZooKeeper | **Present** | Uses `apache/kafka:4.0.0` in combined broker/controller mode. |
| ClickHouse | **Present** | Has a named volume, loopback ports, and a configured default-user password. |
| Vector | **Present, incomplete** | A basic config reads `raw-events` and writes `normalized-events`, but it does not implement the required normalization, quarantine, or ClickHouse ingestion. |
| API, frontend, detector services | **Present, placeholders** | They have Dockerfiles and Compose entries, but the API has no ingestion endpoint, the frontend is static, and the detector does not consume Kafka or expose a health endpoint. |
| Bind every port to `127.0.0.1` | **Present** | All Compose host port mappings bind to loopback. |
| Healthcheck on every service | **Declared, not all valid** | Each service has a healthcheck, but the detector healthcheck calls `--healthcheck`, which the placeholder binary doesn’t implement. The Kafka-init healthcheck relies on a marker file that only exists inside the container. |
| Named volumes | **Present** | Postgres, Kafka, ClickHouse, and Vector use named volumes. |
| Kafka init job and re-run safety | **Present, with a caveat** | Uses `--create --if-not-exists` for all four topics. The init container’s healthcheck may be awkward because the job exits after completion. |
| Disable Kafka auto-topic creation | **Present** | Broker config sets `KAFKA_AUTO_CREATE_TOPICS_ENABLE: "false"`. |
| Topic names, partitions, retention | **Present** | The init script sets 3/3/3/1 partitions and 7/3/7/14-day retention. |
| Topic keys and record formats | **Not implemented** | Kafka topics don’t enforce key formats or payload schemas; producers/configuration and contracts still need to define and use them. |
| Makefile targets | **Present, incomplete** | `up`, `down`, `test`, `lint`, and `fmt` exist, but test/lint/fmt targets invoke tools and scripts not yet set up (for example, no frontend test script). |
| Pre-commit hooks | **Present, minimal** | Basic whitespace/YAML/conflict/private-key hooks only. No language-specific format or lint hooks. |
| `.env.example` | **Present** | Contains local service settings and development credentials. |
| Quickstart README | **Present, but overstates readiness** | It gives setup steps, but the advertised working console/API and end-to-end behavior aren’t implemented yet. |
| Slim profile for 16 GB laptops using W3-4 values | **Partial / unverified** | A slim override exists with lower Postgres/Kafka/ClickHouse memory settings, but I don’t know what “W3-4” refers to and couldn’t verify those values against it. |
| Nix flake and direnv | **Present, unverified** | `flake.nix` and `.envrc` exist, but I haven’t run `nix develop` or generated a `flake.lock`. |
