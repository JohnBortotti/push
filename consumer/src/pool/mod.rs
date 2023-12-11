use bb8::ManageConnection;
use async_trait::async_trait;
use lapin::Channel;

use crate::repository::rabbitmq::RabbitMQRepository;

pub struct AmqpChannelManager {
    pub rabbitmq_repository: RabbitMQRepository,
}
pub struct Resource {
    pub rabbitmq_channel: Channel,
}

#[async_trait]
impl ManageConnection for AmqpChannelManager {
    type Connection = Resource;
    type Error = lapin::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let rabbitmq_channel = self.rabbitmq_repository.create_channel().await?;

        Ok(Resource { rabbitmq_channel })
    }

    async fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}