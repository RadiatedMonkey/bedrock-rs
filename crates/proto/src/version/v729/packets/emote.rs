use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use xuid::Xuid;

#[gamepacket(id = 138)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct EmotePacket {
    pub runtime_id: ActorRuntimeID,
    pub emote_id: String,
    /// Emote length measured in ticks.
    #[endianness(var)]
    pub emote_length: u32,
    pub xuid: Xuid,
    pub platform_id: String,
    // TODO: Turn this into an enum
    pub flags: i8,
}
