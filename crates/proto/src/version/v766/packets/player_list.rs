use crate::version::v766::enums::PlayerListPacketType;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 63)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct PlayerListPacket {
    pub action: PlayerListPacketType,
}