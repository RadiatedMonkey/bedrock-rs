use crate::version::v766::enums::PlayerListPacketType;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 63)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct PlayerListPacket {
    pub action: PlayerListPacketType,
}