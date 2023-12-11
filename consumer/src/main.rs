mod config;
mod email;
mod html_templates;
mod notification;
mod pool;
mod repository;
mod message;

use std::sync::Arc;
use bb8::Pool;
use futures_lite::StreamExt;
use lapin::{types::FieldTable, options::BasicConsumeOptions};
use pool::AmqpChannelManager;
use message::handle_message;

const URL: &'static str = "https://api.sendgrid.com/v3/mail/send";

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let addr = format!(
        "amqp://{}:{}@{}:{}/%2f",
        config.rabbitmq_user, config.rabbitmq_password, config.rabbitmq_ip, config.rabbitmq_port
    );

    let rabbitmq_repository = repository::rabbitmq::RabbitMQRepository::new(&addr).await;

    // ensure the queue and exchange are created and bound
    println!("creating exchange: {}", config.exchange_name);
    println!("creating queue: {}", config.rabbitmq_queue);
    println!("binding queue to exchange");
    let channel = rabbitmq_repository.create_channel().await.unwrap();
    rabbitmq_repository
        .create_exchange(&channel, config.exchange_name.as_str())
        .await
        .unwrap();
    rabbitmq_repository
        .create_queue(&channel, &config.rabbitmq_queue)
        .await
        .unwrap();
    rabbitmq_repository
        .bind_queue_to_exchange(
            &channel,
            &config.rabbitmq_queue,
            config.exchange_name.as_str(),
            "push.notifications.smtp",
        )
        .await
        .unwrap();

    println!("creating channel pool, pool size: {}", config.max_channels);
    let channel_pool = Arc::new(
        Pool::builder()
            .max_size(config.max_channels)
            .build(AmqpChannelManager {
                rabbitmq_repository,
            })
            .await
            .unwrap(),
    );

    // spawn threads to handle messages
    for index in 0..config.max_channels {
        let channel_pool = Arc::clone(&channel_pool);
        let config = config.clone();

        tokio::spawn(async move {
            let channel = channel_pool.get().await.unwrap();
            let mut consumer = channel
                .rabbitmq_channel
                .basic_consume(
                    &config.rabbitmq_queue.clone(),
                    &format!("consumer-{}", index),
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await
                .unwrap();

            while let Some(delivery) = consumer.next().await {
                println!("received message");
                let delivery = delivery.expect("error in consumer");

                match handle_message(
                    delivery,
                    URL,
                    config.sendgrid_api_key.clone(),
                    config.mail_from_email.as_str(),
                    config.mail_from_name.as_str(),
                    config.mail_to.as_str(),
                ).await {
                    Ok(_) => {
                        println!("message handled");
                    }
                    Err(e) => {
                        println!("error: {}", e);
                    }
                }
            }
        });
    }

    // main loop to keep the program running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
