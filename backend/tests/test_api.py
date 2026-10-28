from fastapi.testclient import TestClient

from app import main


def test_healthcheck():
    assert TestClient(main.app).get("/healthz").json() == {"status": "ok"}


def test_batch_publishes_key_and_headers(monkeypatch):
    sent = []

    class Producer:
        def send(self, topic, **kwargs):
            sent.append((topic, kwargs))

        def flush(self, timeout):
            pass

        def close(self):
            pass

    monkeypatch.setattr(main, "kafka_producer", Producer)
    response = TestClient(main.app).post(
        "/batches", json={"source_id": "acct-1", "events": [{"event_id": "e1"}]}
    )
    assert response.status_code == 202
    assert response.json()["accepted"] == 1
    topic, message = sent[0]
    assert topic == "raw-events"
    assert message["key"] == "acct-1"
    assert {name for name, _ in message["headers"]} == {"batch_id", "source_id", "received_at"}
