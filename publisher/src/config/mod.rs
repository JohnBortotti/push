use std::env;

pub struct Config {
    pub rabbitmq_ip: String,
    pub rabbitmq_port: u16,
    pub rabbitmq_user: String,
    pub rabbitmq_password: String,
}

impl Config {
    pub fn new() -> Self {
        let config = Config {
            rabbitmq_ip: env::var("RABBITMQ_IP").expect("RABBITMQ_IP must be set"),
            rabbitmq_port: env::var("RABBITMQ_PORT").expect("RABBITMQ_PORT must be set").parse::<u16>().expect("RABBITMQ_PORT must be a number"),
            rabbitmq_user: env::var("RABBITMQ_USER").expect("RABBITMQ_USER must be set"),
            rabbitmq_password: env::var("RABBITMQ_PASSWORD").expect("RABBITMQ_PASSWORD must be set"),
        };

        config
    }
}