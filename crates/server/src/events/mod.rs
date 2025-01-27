use crate::events::handle::PlayerHandle;

mod events;
mod handle;

pub trait EventListener {
    type Event;

    async fn handle(&self, event: &Self::Event);
}

pub enum Event {
    PlayerSpawn { player: PlayerHandle },
}
