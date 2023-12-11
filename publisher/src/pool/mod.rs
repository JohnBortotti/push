use bb8::ManageConnection;
use lapin::Connection;
use async_trait::async_trait;

pub struct AmqpChannelManager {
    pub connection: Connection,
}

#[async_trait]
impl ManageConnection for AmqpChannelManager {
    type Connection = lapin::Channel;
    type Error = lapin::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.connection.create_channel().await
    }

    async fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}