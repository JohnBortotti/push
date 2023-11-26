mod html_templates;
mod email;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicAckArguments, BasicGetArguments, Channel},
    connection::{Connection, OpenConnectionArguments},
};
use email::SendGridEmail;
use reqwest::Client;
use serde::Deserialize;
use std::{env, thread, time::Duration};
use tokio::sync::Semaphore;
use html_templates::NotificationCategory;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Message {
    title: String,
    description: String,
    category: NotificationCategory,
    timestamp: String,
}

const URL: &'static str = "https://api.sendgrid.com/v3/mail/send";

async fn handle_message(
        email: SendGridEmail, 
        channel: &Channel, 
        ack_args: BasicAckArguments, 
        client: &Client, 
        sendgrid_api_key: String
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let request = client
        .post(URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", sendgrid_api_key))
        .json(&email);
    let response = request.send().await.unwrap();

    if response.status().is_success() {
        println!("Email sent successfully!");
    } else {
        panic!("Failed to send email: {:?}", response.text().await);
    }

    match channel.basic_ack(ack_args).await {
        Ok(_) => {
            dbg!("message ACK completed!");
        }
        Err(err) => {
            dbg!("message ACK error: {}", err);
        }
    };

    println!("");
    println!("");
    Ok(())
}

#[tokio::main]
async fn main() {
    let rabbitmq_ip = env::var("RABBITMQ_IP").expect("env var not found: RABBITMQ_IP");
    let rabbitmq_port = env::var("RABBITMQ_PORT")
        .expect("env var not found: RABBITMQ_PORT")
        .parse::<u16>()
        .expect("env var invalid format: RABBITMQ_PORT");
    let rabbitmq_user = env::var("RABBITMQ_USER").expect("env var not found: RABBITMQ_USER");
    let rabbitmq_password = env::var("RABBITMQ_PASSWORD").expect("env var not found: RABBITMQ_PASSWORD");
    let sendgrid_api_key = env::var("SENDGRID_KEY").expect("env var not found: SENDGRID_KEY");
    let mail_from_email = env::var("MAIL_FROM_EMAIL").expect("env var not found: MAIL_FROM_EMAIL");
    let mail_from_name = env::var("MAIL_FROM_NAME").expect("env var not found: MAIL_FROM_NAME");
    let mail_to = env::var("MAIL_TO").expect("env var not found: MAIL_TO");
    let polling_delay = env::var("POLLING_DELAY").unwrap_or("1".to_string())
        .parse::<u64>()
        .expect("env var invalid format: POOLING_DELAY");
    let semaphore_limit = env::var("ASYNC_TASKS_LIMIT").unwrap_or("5".to_string())
        .parse::<usize>()
        .expect("env var invalid format: ASYNC_TASKS_LIMIT");

    let conn = Connection::open(&OpenConnectionArguments::new(
        &rabbitmq_ip,
        rabbitmq_port,
        &rabbitmq_user,
        &rabbitmq_password,
    ))
    .await
    .expect("Failed to open connection");

    conn.register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    let channel = conn.open_channel(None).await.expect("Failed to open channel");
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .expect("Failed to register channel callback");

    let args = BasicGetArguments::new("notifications-queue");
    let client = Client::new();

    let semaphore = Arc::new(Semaphore::new(semaphore_limit));

    loop {
        let semaphore_clone = Arc::clone(&semaphore);
        match channel.basic_get(args.clone()).await {
            Ok(message) => match message {
                Some(get_message) => {
                    let message_data = String::from_utf8(get_message.2).unwrap();
                    let message_json: Message = serde_json::from_str(&message_data).unwrap();
                    let ack_args = BasicAckArguments::new(get_message.0.delivery_tag(), false);
                    let email = email::build_sendgrid(&mail_from_email, &mail_from_name, &mail_to, message_json);

                    let channel_clone = channel.clone();
                    let client_clone = client.clone();
                    let sendgrid_api_key_clone = sendgrid_api_key.clone();

                    tokio::spawn(async move {
                        let permit = semaphore_clone.acquire().await.unwrap();
                        let _ = handle_message(email, &channel_clone, ack_args, &client_clone, sendgrid_api_key_clone).await;
                        drop(permit);
                    });
                }
                None => {}
            },
            Err(err) => {
                println!(
                    "Consumer error, queue: {}, err: {}",
                    "notifications-queue", err
                )
            }
        }
        thread::sleep(Duration::from_secs(polling_delay));
    }
}
