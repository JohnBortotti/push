use std::env;

pub struct Config {
    pub rabbitmq_ip: String,
    pub rabbitmq_port: u16,
    pub rabbitmq_user: String,
    pub rabbitmq_password: String,
    pub max_channels: u32,
    pub exchange_name: String,
}

impl Config {
    pub fn new() -> Self {
        let config = Config {
            rabbitmq_ip: env::var("RABBITMQ_IP").expect("RABBITMQ_IP must be set"),
            rabbitmq_port: env::var("RABBITMQ_PORT").expect("RABBITMQ_PORT must be set").parse::<u16>().expect("RABBITMQ_PORT must be a number"),
            rabbitmq_user: env::var("RABBITMQ_USER").expect("RABBITMQ_USER must be set"),
            rabbitmq_password: env::var("RABBITMQ_PASSWORD").expect("RABBITMQ_PASSWORD must be set"),
            max_channels: env::var("MAX_CHANNELS").unwrap_or("10".to_string()).parse::<u32>().expect("MAX_CHANNELS must be a number"),
            exchange_name: env::var("EXCHANGE_NAME").unwrap_or("push.notifications".to_string()),
        };

        config
    }
}