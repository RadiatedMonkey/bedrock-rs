use bedrockrs_proto::listener::Listener;
use std::sync::Arc;
use tokio::sync::{oneshot::Sender, Notify};

pub struct ServerHandle {
    shutdown_sender: Sender<ShutdownKind>,
    shutdown_notify: Arc<Notify>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ShutdownKind {
    Graceful,
    Forceful,
}

impl ServerHandle {
    pub fn new(
        shutdown_sender: Sender<ShutdownKind>,
        shutdown_notify: Arc<Notify>,
    ) -> ServerHandle {
        Self {
            shutdown_sender,
            shutdown_notify,
        }
    }

    /// Initiates a graceful shutdown and waits for completion.
    pub async fn shutdown_graceful(self) {
        let _ = self.shutdown_sender.send(ShutdownKind::Graceful);
        self.shutdown_notify.notified().await;
    }

    /// Initiates a forceful shutdown and waits for completion.
    pub async fn shutdown_forceful(self) {
        let _ = self.shutdown_sender.send(ShutdownKind::Forceful);
        self.shutdown_notify.notified().await;
    }

    /// Initiates a graceful shutdown without waiting for completion.
    pub fn shutdown_graceful_now(self) {
        let _ = self.shutdown_sender.send(ShutdownKind::Graceful);
    }

    /// Initiates a forceful shutdown without waiting for completion.
    pub fn shutdown_forceful_now(self) {
        let _ = self.shutdown_sender.send(ShutdownKind::Forceful);
    }

    pub fn add_listener(&mut self, listener: Listener) {
        todo!()
    }
}
