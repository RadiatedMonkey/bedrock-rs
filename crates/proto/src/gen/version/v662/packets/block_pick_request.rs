use crate::version::v662::types::BlockPos;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 34)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct BlockPickRequestPacket {
    pub position: BlockPos,
    pub with_data: bool,
    pub max_slots: i8,
}