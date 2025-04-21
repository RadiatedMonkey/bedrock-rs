use crate::version::v662::enums::PlayStatus;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 2)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct PlayStatusPacket {
    pub status: PlayStatus,
}