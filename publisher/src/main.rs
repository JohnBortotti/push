#[macro_use]
extern crate rocket;

mod config;
mod notification;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::BasicPublishArguments,
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use chrono::{DateTime, Local};
use notification::Notification;
use rocket::{State, serde::json::Json};
use tokio;
use tokio::io::Error as TError;

#[post("/notify", format = "application/json", data = "<notification>")]
async fn notify(notification: Json<Notification<'_>>, config: &State<config::Config>) -> Result<(), TError> {
    let conn = Connection::open(&OpenConnectionArguments::new(
        &config.rabbitmq_ip,
        config.rabbitmq_port,
        &config.rabbitmq_user,
        &config.rabbitmq_password,
    ))
    .await
    .expect("Failed to open connection");

    conn.register_callback(DefaultConnectionCallback)
        .await
        .expect("Failed to register connection callback");

    let channel = conn.open_channel(None).await.expect("Failed to open channel");
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .expect("Failed to register channel callback");

    let rounting_key = "notifications.smtp";
    let exchange_name = "notifications";

    let local_time: DateTime<Local> = Local::now();
    let local_time_str = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

    let content = serde_json::json!({
        "title": notification.title,
        "description": notification.description,
        "category": notification.category,
        "timestamp": local_time_str
    }).to_string().into_bytes();

    let args = BasicPublishArguments::new(exchange_name, rounting_key);

    channel
        .basic_publish(BasicProperties::default(), content, args)
        .await
        .expect("Failed to publish message");

    channel.close().await.unwrap();
    conn.close().await.unwrap();

    Ok(())
}

#[launch]
fn rocket() -> _ {
    let config = config::Config::new();

    rocket::build().manage(config).mount("/", routes![notify])
}
