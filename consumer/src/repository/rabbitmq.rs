use lapin::{
    Channel, Connection, ConnectionProperties, Error,
};

pub struct RabbitMQRepository {
    pub transport: Connection,
}

impl RabbitMQRepository {
    pub async fn new(addr: &str) -> RabbitMQRepository {
        let transport = Connection::connect(addr, ConnectionProperties::default())
            .await
            .unwrap();

        RabbitMQRepository { transport }
    }

    pub async fn create_channel(&self) -> Result<Channel, Error> {
        self.transport.create_channel().await
    }

    pub async fn create_exchange(&self, channel: &Channel, exchange_name: &str) -> Result<(), Error> {
        channel
            .exchange_declare(
                exchange_name,
                lapin::ExchangeKind::Topic,
                lapin::options::ExchangeDeclareOptions::default(),
                lapin::types::FieldTable::default(),
            )
            .await
    }

    pub async fn create_queue(&self, channel: &Channel, queue_name: &str) -> Result<(), Error> {
        let _ = channel
            .queue_declare(
                queue_name,
                lapin::options::QueueDeclareOptions::default(),
                lapin::types::FieldTable::default(),
            )
            .await;

        Ok(())
    }

    pub async fn bind_queue_to_exchange(
        &self,
        channel: &Channel,
        queue_name: &str,
        exchange_name: &str,
        routing_key: &str,
    ) -> Result<(), Error> {
        channel
            .queue_bind(
                queue_name,
                exchange_name,
                routing_key,
                lapin::options::QueueBindOptions::default(),
                lapin::types::FieldTable::default(),
            )
            .await
    }

}