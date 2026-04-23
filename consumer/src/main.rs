use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::Message;
use tokio_stream::StreamExt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Data {
    temp: u8,
}

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "test-group")
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["hygiene"]).unwrap();

    let mut stream = consumer.stream();

    while let Some(message) = stream.next().await {
        match message {
            Ok(m) => {
                if let Some(payload) = m.payload() {
                    let text = std::str::from_utf8(payload).unwrap();

                    if let Ok(data) = serde_json::from_str::<Data>(text) {
                        println!("Temp: {}", data.temp);

                        if data.temp > 30 {
                            println!("⚠️ High temperature alert!");
                        }
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
