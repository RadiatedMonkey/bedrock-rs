mod events;

use crate::v662::types::ActorRuntimeID;
use crate::v748;
use xuid::Xuid;
use crate::adapter::events::emote::EmotePacketEvent;

pub enum GamePacketEvents {
    EmoteEvent(EmotePacketEvent),
}

impl From<v748::gamepackets::GamePackets> for GamePacketEvents {
    fn from(packet: v748::gamepackets::GamePackets) -> Self {
        match packet {
            // v748::gamepackets::GamePackets::Emote(packet) => {
            //     GamePacketEvents::EmoteEvent(EmotePacketEvent {
            //         actor_runtime_id: packet.actor_runtime_id,
            //         emote_id: packet.emote_id,
            //         xuid: packet.xuid,
            //     })
            // }

            _ => todo!(),
        }
    }
}
