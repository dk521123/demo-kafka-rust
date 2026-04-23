use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");

    loop {
        let temp = rand::random::<u8>() % 40;
        let payload = format!(r#"{{"temp": {}}}"#, temp);

        let _ = producer.send(
            FutureRecord::to("hygiene")
                .payload(&payload)
                .key("key"),
            Duration::from_secs(0),
        ).await;

        println!("Sent: {}", payload);
        sleep(Duration::from_secs(2)).await;
    }
}
