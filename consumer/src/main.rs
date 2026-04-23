use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::Message;
use tokio_stream::StreamExt;
use serde::Deserialize;
use reqwest::Client;

#[derive(Debug, Deserialize)]
struct Data {
    temp: f64,
}

#[tokio::main]
async fn main() {
    // HTTP client（InfluxDB用）
    let http = Client::new();

    // Kafka consumer
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "test-group")
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["hygiene"]).unwrap();
    let mut stream = consumer.stream();

    println!("🚀 consuming...");

    while let Some(message) = stream.next().await {
        match message {
            Ok(m) => {
                if let Some(payload) = m.payload() {
                    if let Ok(text) = std::str::from_utf8(payload) {
                        if let Ok(data) = serde_json::from_str::<Data>(text) {
                            println!("Temp: {}", data.temp);

                            // アラート
                            if data.temp > 30.0 {
                                println!("⚠️ High temperature alert!");
                            }

                            // InfluxDBへ送信（HTTP Line Protocol）
                            let line = format!("sensor temp={}", data.temp);

                            let res = http
                                .post("http://localhost:8086/api/v2/write?org=demo&bucket=hygiene&precision=s")
                                .header("Authorization", "Token mytoken")
                                .body(line)
                                .send()
                                .await;

                            match res {
                                Ok(r) => println!("written to DB: {}", r.status()),
                                Err(e) => println!("write error: {}", e),
                            }
                        }
                    }
                }
            }
            Err(e) => println!("Kafka error: {}", e),
        }
    }
}