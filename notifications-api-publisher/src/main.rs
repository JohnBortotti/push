#[macro_use]
extern crate rocket;
use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::BasicPublishArguments,
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use chrono::{DateTime, Local};
use rocket::serde::{json::Json, Deserialize, Serialize};
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

#[post("/notify", format = "application/json", data = "<notification>")]
async fn notify(notification: Json<Notification<'_>>) -> Result<(), TError> {
    let conn = Connection::open(&OpenConnectionArguments::new(
        &env::var("RABBITMQ_IP").unwrap(),
        env::var("RABBITMQ_PORT").unwrap().parse::<u16>().unwrap(),
        &env::var("RABBITMQ_USER").unwrap(),
        &env::var("RABBITMQ_PASSWORD").unwrap(),
    ))
    .await
    .unwrap();

    conn.register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    let channel = conn.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    let rounting_key = "notifications.smtp";
    let exchange_name = "notifications";

    let local_time: DateTime<Local> = Local::now();
    let local_time_str = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

    let content = format!(
        r#"{{ "title": "{}", "description": "{}", "category": "{}", "timestamp": "{}" }}"#,
        notification.title, notification.description, notification.category, local_time_str
    )
    .into_bytes();

    let args = BasicPublishArguments::new(exchange_name, rounting_key);

    channel
        .basic_publish(BasicProperties::default(), content, args)
        .await
        .unwrap();

    channel.close().await.unwrap();
    conn.close().await.unwrap();

    Ok(())
}

#[launch]
fn rocket() -> _ {
    // validate env
    env::var("RABBITMQ_IP").expect("env var not found: RABBITMQ_IP");
    env::var("RABBITMQ_PORT")
        .expect("env var not found: RABBITMQ_PORT")
        .parse::<u16>()
        .expect("env var invalid format: RABBITMQ_PORT");
    env::var("RABBITMQ_USER").expect("env var not found: RABBITMQ_USER");
    env::var("RABBITMQ_PASSWORD").expect("env var not found: RABBITMQ_PASSWORD");

    // register route
    rocket::build().mount("/", routes![notify])
}
