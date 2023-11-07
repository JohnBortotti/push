use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicAckArguments, BasicGetArguments},
    connection::{Connection, OpenConnectionArguments},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, thread, time::Duration};
use tokio;

mod html_templates;

#[derive(Serialize)]
struct SendGridEmail {
    personalizations: Vec<Personalization>,
    from: From,
    subject: String,
    content: Vec<Content>,
}

#[derive(Serialize)]
struct Personalization {
    to: Vec<To>,
}

#[derive(Serialize)]
struct To {
    email: String,
}

#[derive(Serialize)]
struct From {
    email: String,
    name: String,
}

#[derive(Serialize)]
struct Content {
    r#type: String,
    value: String,
}

#[derive(Deserialize)]
pub struct Message {
    title: String,
    description: String,
    category: NotificationCategory,
    timestamp: String,
}

#[derive(Deserialize)]
pub enum NotificationCategory {
    Alert,
    Critical,
    Report,
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
    let polling_delay = env::var("POLLING_DELAY").expect("env var not found: POLLING_DELAY")
        .parse::<u64>()
        .expect("env var invalid format: POOLING_DELAY");

    let conn = Connection::open(&OpenConnectionArguments::new(
        &rabbitmq_ip,
        rabbitmq_port,
        &rabbitmq_user,
        &rabbitmq_password,
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

    let args = BasicGetArguments::new("notifications-queue");

    let client = Client::new();
    let url = "https://api.sendgrid.com/v3/mail/send";

    loop {
        match channel.basic_get(args.clone()).await {
            Ok(message) => match message {
                Some(get_message) => {
                    let message_data = String::from_utf8(get_message.2).unwrap();
                    let message_json: Message = serde_json::from_str(&message_data).unwrap();

                    let ack_args = BasicAckArguments::new(get_message.0.delivery_tag(), false);

                    let email = SendGridEmail {
                        from: From {
                            email: mail_from_email.to_string(),
                            name: mail_from_name.to_string(),
                        },
                        personalizations: vec![Personalization {
                            to: vec![To {
                                email: mail_to.to_string(),
                            }],
                        }],
                        subject: message_json.title.clone(),
                        content: vec![Content {
                            r#type: "text/html".to_string(),
                            value: html_templates::build(message_json),
                        }],
                    };

                    let request = client
                        .post(url)
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
                }
                None => {
                    dbg!("No messages");
                }
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
