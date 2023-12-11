#[macro_use]
extern crate rocket;

mod config;
mod notification;
mod pool;

use bb8::Pool;
use bb8_lapin::prelude::*;
use chrono::{DateTime, Local};
use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Connection, ConnectionProperties};
use notification::Notification;
use pool::AmqpChannelManager;
use rocket::{serde::json::Json, State};
use tokio;
use tokio::io::Error as TError;

#[post("/notify", format = "application/json", data = "<notification>")]
async fn notify(
    notification: Json<Notification<'_>>,
    pool: &State<Pool<AmqpChannelManager>>,
    config: &State<config::Config>,
) -> Result<(), TError> {
    let channel = pool.get().await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let rounting_key = "notifications.smtp";

    let local_time: DateTime<Local> = Local::now();
    let local_time_str = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

    let content = serde_json::json!({
        "title": notification.title,
        "description": notification.description,
        "category": notification.category,
        "timestamp": local_time_str
    })
    .to_string()
    .into_bytes();

    channel
        .basic_publish(
            config.exchange_name.as_str(),
            rounting_key,
            BasicPublishOptions::default(),
            &content,
            BasicProperties::default(),
        )
        .await
        .expect("Failed to publish message");

    Ok(())
}

#[launch]
async fn rocket() -> _ {
    let config = config::Config::new();

    let addr = format!(
        "amqp://{}:{}@{}:{}/%2f",
        config.rabbitmq_user, config.rabbitmq_password, config.rabbitmq_ip, config.rabbitmq_port
    );
    let connection = Connection::connect(&addr, ConnectionProperties::default())
        .await
        .unwrap();
    let manager = AmqpChannelManager { connection };

    let pool = Pool::builder()
        .max_size(config.max_channels)
        .build(manager)
        .await
        .unwrap();

    rocket::build()
        .manage(pool)
        .manage(config)
        .mount("/", routes![notify])
}
