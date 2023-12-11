#[macro_use]
extern crate rocket;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::BasicPublishArguments,
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use chrono::{DateTime, Local};
use rocket::{State, serde::{json::Json, Deserialize, Serialize}};
use std::env;
use std::fmt;
use tokio;
use tokio::io::Error as TError;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Notification<'r> {
    title: &'r str,
    description: &'r str,
    category: NotificationCategory,
}

#[derive(Deserialize, Serialize)]
pub enum NotificationCategory {
    Alert,
    Critical,
    Report,
}

impl fmt::Display for NotificationCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotificationCategory::Alert => write!(f, "Alert"),
            NotificationCategory::Critical => write!(f, "Critical"),
            NotificationCategory::Report => write!(f, "Report"),
        }
    }
}

struct Config {
    rabbitmq_ip: String,
    rabbitmq_port: u16,
    rabbitmq_user: String,
    rabbitmq_password: String,
}

#[post("/notify", format = "application/json", data = "<notification>")]
async fn notify(notification: Json<Notification<'_>>, config: &State<Config>) -> Result<(), TError> {
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
    let config = Config {
        rabbitmq_ip: env::var("RABBITMQ_IP").expect("RABBITMQ_IP must be set"),
        rabbitmq_port: env::var("RABBITMQ_PORT").expect("RABBITMQ_PORT must be set").parse::<u16>().expect("RABBITMQ_PORT must be a number"),
        rabbitmq_user: env::var("RABBITMQ_USER").expect("RABBITMQ_USER must be set"),
        rabbitmq_password: env::var("RABBITMQ_PASSWORD").expect("RABBITMQ_PASSWORD must be set"),
    };

    rocket::build().manage(config).mount("/", routes![notify])
}
