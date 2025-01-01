use crate::connection::Connection;
use crate::error::ConnectionError;
use crate::helper::ProtoHelper;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

pub fn shard<T: ProtoHelper>(connection: Connection) -> ConnectionShared<T> {
    ConnectionShared::<T> {
        connection: Arc::new(RwLock::new(connection)),
        queue_send: Arc::new(RwLock::new(Vec::new())),
        queue_recv: Arc::new(RwLock::new(VecDeque::new())),
    }
}

#[derive(Clone)]
pub struct ConnectionShared<T: ProtoHelper> {
    connection: Arc<RwLock<Connection>>,
    queue_send: Arc<RwLock<Vec<T::GamePacketType>>>,
    queue_recv: Arc<RwLock<VecDeque<T::GamePacketType>>>,
}

impl<T: ProtoHelper> ConnectionShared<T> {
    pub async fn write(&mut self, gamepacket: T::GamePacketType) -> Result<(), ConnectionError> {
        let mut queue_send = self.queue_send.write().await;

        queue_send.push(gamepacket);

        Ok(())
    }

    pub async fn read(&mut self) -> Option<T::GamePacketType> {
        let mut queue_recv = self.queue_recv.write().await;

        queue_recv.pop_front()
    }

    pub async fn read_all(&mut self) -> Vec<T::GamePacketType> {
        let mut queue_recv = self.queue_recv.write().await;
        queue_recv.make_contiguous();
        queue_recv.drain(..).collect::<Vec<_>>()
    }

    pub async fn send(&mut self) -> Result<(), ConnectionError> {
        let mut gamepackets = self.queue_send.write().await;
        let mut conn = self.connection.write().await;

        conn.send::<T>(gamepackets.as_slice()).await?;

        gamepackets.clear();

        Ok(())
    }

    pub async fn recv(&mut self) -> Result<(), ConnectionError> {
        let mut conn = self.connection.write().await;

        let gamepackets = conn.recv::<T>().await?;

        if !gamepackets.is_empty() {
            let mut queue_recv = self.queue_recv.write().await;

            for gamepacket in gamepackets {
                queue_recv.push_back(gamepacket);
            }
        }

        Ok(())
    }
}
