use crate::server::Server;
use crate::systems::movement::movement_system;
use crate::{ServerHandle, ShutdownKind};
use bedrockrs_proto::listener::Listener;
use shipyard::{IntoWorkload, World};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::{oneshot, Notify};

pub struct ServerBuilder {
    name: String,
    sub_name: String,
    listeners_info: Vec<SocketAddr>,
    max_player: u32,
}

impl ServerBuilder {
    pub fn new() -> ServerBuilder {
        Self::default()
    }

    pub fn name(mut self, name: &str) -> ServerBuilder {
        self.name = name.to_owned();
        self
    }

    pub fn sub_name(mut self, sub_name: &str) -> ServerBuilder {
        self.sub_name = sub_name.to_owned();
        self
    }

    pub fn listener(mut self, addr: SocketAddr) -> ServerBuilder {
        self.listeners_info.push(addr);
        self
    }

    pub async fn build(self) -> (Server, ServerHandle) {
        let mut listeners = Vec::with_capacity(self.listeners_info.len());

        for addr in self.listeners_info {
            listeners.push(
                Listener::new_raknet(
                    self.name.clone(),
                    self.sub_name.clone(),
                    String::from("1.21.0"),
                    self.max_player,
                    0,
                    addr,
                    false,
                )
                .await
                .unwrap(),
            )
        }

        let world = World::new();

        world.add_workload(|| (movement_system).into_workload());

        let notify = Arc::new(Notify::new());
        let (sender, receiver) = oneshot::channel::<ShutdownKind>();

        let server = Server {
            listeners,
            world,
            shutdown_notify: notify.clone(),
            shutdown_recv: receiver,
        };

        let handle = ServerHandle::new(sender, notify.clone());

        (server, handle)
    }
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self {
            name: "bedrock-server".to_string(),
            sub_name: "bedrock-rs".to_string(),
            listeners_info: vec![],
            max_player: 100,
        }
    }
}
