use std::env;

#[derive(Clone)]
pub struct Config {
    pub rabbitmq_ip: String,
    pub rabbitmq_port: u16,
    pub rabbitmq_user: String,
    pub rabbitmq_password: String,
    pub rabbitmq_queue: String,
    pub exchange_name: String,
    pub sendgrid_api_key: String,
    pub mail_from_email: String,
    pub mail_from_name: String,
    pub mail_to: String,
    pub max_channels: u32,
}

impl Config {
    pub fn new() -> Self {
        let config = Config {
            rabbitmq_ip: env::var("RABBITMQ_IP").expect("RABBITMQ_IP must be set"),
            rabbitmq_port: env::var("RABBITMQ_PORT")
                .expect("RABBITMQ_PORT must be set")
                .parse::<u16>()
                .expect("RABBITMQ_PORT must be a number"),
            rabbitmq_user: env::var("RABBITMQ_USER").expect("RABBITMQ_USER must be set"),
            rabbitmq_password: env::var("RABBITMQ_PASSWORD")
                .expect("RABBITMQ_PASSWORD must be set"),
            rabbitmq_queue: env::var("RABBITMQ_QUEUE")
                .unwrap_or("push-notifications-queue".to_string()),
            exchange_name: env::var("EXCHANGE_NAME").unwrap_or("push.notifications".to_string()),
            sendgrid_api_key: env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set"),
            mail_from_email: env::var("MAIL_FROM_EMAIL").expect("MAIL_FROM_EMAIL must be set"),
            mail_from_name: env::var("MAIL_FROM_NAME").expect("MAIL_FROM_NAME must be set"),
            mail_to: env::var("MAIL_TO").expect("MAIL_TO must be set"),
            max_channels: env::var("MAX_CHANNELS")
                .unwrap_or("10".to_string())
                .parse::<u32>()
                .expect("MAX_CHANNELS must be a number"),
        };

        config
    }
}
