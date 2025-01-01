use crate::error::LoginError;
use bedrockrs_proto::listener::Listener;
use shipyard::World;
use std::error::Error;

pub mod builder;

pub struct Server {
    listeners: Vec<Listener>,
    name: String,
    sub_name: String,
    pub world: World,
}

impl Server {
    pub async fn start(&mut self) {
        for listener in &mut self.listeners {
            listener.start().await.unwrap();
        }

        self.world.run_default_workload().unwrap()
    }

    pub async fn stop(&mut self) {
        for listener in &mut self.listeners {
            listener.stop().await.expect("TODO: panic message");
        }
    }

    pub async fn accept(&mut self) -> Result<(), LoginError> {
        todo!()
    }
}
