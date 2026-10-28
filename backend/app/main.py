import json
import os
import uuid
from datetime import UTC, datetime
from typing import Any

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI(title="Signal Pipeline API")


class Batch(BaseModel):
    source_id: str
    events: list[dict[str, Any]]


def kafka_producer():
    from kafka import KafkaProducer

    return KafkaProducer(
        bootstrap_servers=os.getenv("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092"),
        key_serializer=str.encode,
        value_serializer=lambda item: json.dumps(item).encode(),
    )


@app.get("/healthz")
def healthz() -> dict[str, str]:
    return {"status": "ok"}


@app.post("/batches", status_code=202)
def submit_batch(batch: Batch) -> dict[str, Any]:
    batch_id = str(uuid.uuid4())
    received_at = datetime.now(UTC).isoformat()
    producer = None
    try:
        producer = kafka_producer()
        for event in batch.events:
            producer.send(
                "raw-events",
                key=batch.source_id,
                value={
                    "batch_id": batch_id,
                    "source_id": batch.source_id,
                    "received_at": received_at,
                    "record": event,
                },
                headers=[
                    ("batch_id", batch_id.encode()),
                    ("source_id", batch.source_id.encode()),
                    ("received_at", received_at.encode()),
                ],
            )
        producer.flush(timeout=10)
    except Exception as exc:
        raise HTTPException(status_code=503, detail=f"Kafka unavailable: {exc}") from exc
    finally:
        if producer is not None:
            producer.close()
    return {"batch_id": batch_id, "accepted": len(batch.events)}
