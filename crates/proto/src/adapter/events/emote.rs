use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use xuid::Xuid;

pub struct EmotePacketEvent {
    pub actor_runtime_id: ActorRuntimeID,
    pub emote_id: String,
    pub xuid: u64,
}

impl From<crate::v729::packets::emote::EmotePacket> for EmotePacketEvent {
    fn from(packet: crate::v729::packets::emote::EmotePacket) -> Self {
        Self {
            actor_runtime_id: packet.runtime_id,
            emote_id: packet.emote_id,
            xuid: packet.xuid.into(),
        }
    }
}
