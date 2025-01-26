use bedrockrs_proto::listener::Listener;
use shipyard::World;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{oneshot::Receiver, Notify};
use tokio::time::{sleep, Instant};

mod builder;
mod handle;

pub use builder::*;
pub use handle::*;

pub struct Server {
    listeners: Vec<Listener>,
    world: World,
    shutdown_notify: Arc<Notify>,
    shutdown_recv: Receiver<ShutdownKind>
}

impl Server {
    const TICKS_PER_SECOND: u64 = 20;
    const TICK_DURATION: Duration = Duration::from_secs(1 / Self::TICKS_PER_SECOND);

    pub async fn run(&mut self) {
        self.load().await;
        
        for listener in &mut self.listeners {
            listener.start().await.unwrap();
        }
        
        loop {
            if let Ok(kind) = self.shutdown_recv.try_recv() {
                if kind == ShutdownKind::Graceful {
                    self.save().await;
                }
                
                break;
            };

            let tick_start = Instant::now();

            self.tick();
            
            println!("TICK");

            let elapsed = tick_start.elapsed();
            
            if elapsed < Self::TICK_DURATION {
                sleep(Self::TICK_DURATION - elapsed).await;
            }
        }
        
        self.shutdown_notify.notify_one();
    }
    
    fn tick(&mut self) {
        self.world.run_default_workload().unwrap()
    }

    async fn load(&mut self) {}

    async fn save(&mut self) {}
}
