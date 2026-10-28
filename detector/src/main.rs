use axum::{routing::get, Json, Router};
use rdkafka::{
    config::ClientConfig,
    consumer::{BaseConsumer, Consumer},
    producer::{BaseProducer, BaseRecord},
    Message,
};
use serde_json::{json, Value};
use std::{env, net::SocketAddr, time::Duration};
use uuid::Uuid;

async fn health() -> Json<&'static str> {
    Json("ok")
}

fn matches_threshold(event: &Value, threshold: f64) -> bool {
    event
        .pointer("/payload/amount")
        .and_then(Value::as_f64)
        .is_some_and(|amount| amount >= threshold)
}

fn kafka_loop() -> Result<(), Box<dyn std::error::Error>> {
    let brokers = env::var("KAFKA_BOOTSTRAP_SERVERS").unwrap_or_else(|_| "localhost:9092".into());
    let threshold = env::var("DETECTOR_AMOUNT_THRESHOLD")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10_000.0);
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", &brokers)
        .set("group.id", "phase1-detector")
        .set("enable.auto.commit", "true")
        .create()?;
    consumer.subscribe(&["normalized-events"])?;
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", &brokers)
        .create()?;
    loop {
        if let Some(message) = consumer.poll(Duration::from_millis(250)) {
            let message = message?;
            let raw = message.payload().unwrap_or_default();
            let event: Value = match serde_json::from_slice(raw) {
                Ok(value) => value,
                Err(_) => {
                    let record = json!({"error":"invalid normalized event", "record":String::from_utf8_lossy(raw)}).to_string();
                    producer
                        .send(
                            BaseRecord::to("quarantine")
                                .key(message.key().unwrap_or_default())
                                .payload(&record),
                        )
                        .map_err(|(e, _)| e)?;
                    producer.poll(Duration::ZERO);
                    continue;
                }
            };
            if !matches_threshold(&event, threshold) {
                continue;
            }
            let signal_id = Uuid::new_v4().to_string();
            let signal = json!({"signal_id":signal_id,"source_id":event.get("source_id").or_else(||event.get("account_id")),"event_id":event.get("event_id"),"rule_id":"high-amount","detected_at":chrono_now(),"payload":event});
            producer
                .send(
                    BaseRecord::to("detection-signals")
                        .key(&signal_id)
                        .payload(&signal.to_string()),
                )
                .map_err(|(e, _)| e)?;
            producer.poll(Duration::ZERO);
        }
    }
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}

#[tokio::main]
async fn main() {
    std::thread::spawn(|| {
        if let Err(error) = kafka_loop() {
            eprintln!("detector Kafka loop stopped: {error}");
        }
    });
    let app = Router::new().route("/healthz", get(health));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind health listener");
    axum::serve(listener, app)
        .await
        .expect("serve health endpoint");
}

#[cfg(test)]
mod tests {
    use super::matches_threshold;
    use serde_json::json;

    #[test]
    fn matches_high_amount_rule() {
        assert!(matches_threshold(
            &json!({"payload":{"amount":100.0}}),
            50.0
        ));
        assert!(!matches_threshold(
            &json!({"payload":{"amount":10.0}}),
            50.0
        ));
        assert!(!matches_threshold(&json!({"payload":{}}), 50.0));
    }
}
