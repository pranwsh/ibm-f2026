# Kafka topics

`kafka-init` creates topics on startup. The initialization script uses `--if-not-exists`, and the broker has automatic topic creation disabled.

| Topic | Key | Partitions | Retention | Value |
|---|---|---:|---:|---|
| `raw-events` | Source account | 3 | 7 days | Original record; headers: `batch_id`, `source_id`, `received_at` |
| `normalized-events` | Source account | 3 | 3 days | `NormalizedEvent` JSON |
| `detection-signals` | `signal_id` | 3 | 7 days | `DetectionSignal` JSON |
| `quarantine` | Source account | 1 | 14 days | Original record and error |

Topic creation configures names, partition counts, and retention. Producers and consumers still need to implement the documented keys, headers, and record formats; these are not enforced by the Kafka broker.
