use futures_lite::stream::StreamExt;
use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Result,
};
use prost::Message;
use rust_protobuf_poc::test::{GreetingMessage, Who};
use tracing::info;

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    // amqp://admin:admin@192.168.1.189:5672
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://admin:admin@localhost:5672/test".into());

    async_global_executor::block_on(async {
        let conn = Connection::connect(
            &addr,
            ConnectionProperties::default(),
        )
        .await?;

        info!("CONNECTED");

        let channel_a = conn.create_channel().await?;
        let channel_b = conn.create_channel().await?;

        let queue = channel_a
            .queue_declare(
                "test1",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!(?queue, "Declared queue");

        /*
        let mut consumer = channel_b
            .basic_consume(
                "test1",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        async_global_executor::spawn(async move {
            info!("will consume");
            while let Some(delivery) = consumer.next().await {
                let delivery = delivery.expect("error in consumer");
                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("ack");
            }
        }).detach();
         */

         let message = GreetingMessage {
          greeting: "helloaaaaa".to_owned(),
          who: Who::World.into(),
         };

         let mut buf = Vec::new();
          buf.reserve(message.encoded_len());
          // Unwrap is safe, since we have reserved sufficient capacity in the vector.
          message.encode(&mut buf).unwrap();

          println!("{:?}", buf);

        let payload = b"Hello world!";

        loop {
            let confirm = channel_a
                .basic_publish(
                    "",
                    "test1",
                    BasicPublishOptions::default(),
                    &buf,
                    BasicProperties::default(),
                )
                .await?
                .await?;
            assert_eq!(confirm, Confirmation::NotRequested);
        }
    })
}