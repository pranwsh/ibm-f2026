CREATE TABLE IF NOT EXISTS analytics.events (received_at DateTime, source_id String, payload String) ENGINE = MergeTree ORDER BY (received_at, source_id);
