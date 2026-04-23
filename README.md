# 
## Pre-conditions
1. Install docker/docker-compose
2. Install Rust

## How to run

1. `docker compose up -d`
2. Command below
```sh
docker ps

# To create a topic (demo-kafka-rust-kafka-1 is container name)
docker exec -it demo-kafka-rust-kafka-1 kafka-topics \
  --create \
  --topic hygiene \
  --bootstrap-server localhost:9092 \
  --partitions 1 \
  --replication-factor 1

# To confirm the topic
docker exec -it demo-kafka-rust-kafka-1 kafka-topics \
  --list \
  --bootstrap-server localhost:9092
```
3. Run the consumer to preceive data through Kafka
```sh
$ cd consumer
$ cargo run
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
   Running `target/debug/consumer`
```

4. Open another console and run the producer to send data through Kafka
```sh
$ cd producer
$ cargo run
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
   Running `target/debug/producer`
Sent: {"temp": 39} # Starting to send data
Sent: {"temp": 2}
...
```

5. Check the console for the consumer, then you can see 
```sh
Temp: 39
⚠️ High temperature alert!
Temp: 2
```

# Appendix-A: How to create this project
## For the producer
```
cargo new producer
cd producer
```
## For the consumer
```
cd ..
cargo new consumer
cd producer
```
