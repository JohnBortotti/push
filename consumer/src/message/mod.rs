use lapin::{message::Delivery, options::BasicAckOptions};
use reqwest::Client;

use crate::{email, notification::Notification};

pub async fn handle_message(
    delivery: Delivery,
    url: &str,
    sendgrid_api_key: String,
    mail_from_email: &str,
    mail_from_name: &str,
    mail_to: &str,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let message = String::from_utf8(delivery.data.clone()).unwrap();
    let message_json: Notification = serde_json::from_str(&message).unwrap();
    let email = email::build_sendgrid(mail_from_email, mail_from_name, mail_to, message_json);

    let request = Client::new()
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

    match delivery.ack(BasicAckOptions::default()).await {
        Ok(_) => {
            println!("message ACK completed!");
        }
        Err(err) => {
            println!("message ACK error: {}", err);
        }
    };

    Ok(())
}
