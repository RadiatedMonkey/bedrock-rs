use crate::v662::types::ActorRuntimeID;
use xuid::Xuid;

pub enum GamePacketEvents {
    EmoteEvent(EmotePacketEvent),
}

pub struct EmotePacketEvent {
    pub actor_runtime_id: ActorRuntimeID,
    pub emote_id: String,
    pub xuid: Xuid,
}

impl From<crate::v748::gamepackets::GamePackets> for GamePacketEvents {
    fn from(packet: crate::v748::gamepackets::GamePackets) -> Self {
        todo!()
    }
}
