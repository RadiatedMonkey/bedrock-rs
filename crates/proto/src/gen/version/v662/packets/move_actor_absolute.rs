use crate::version::v662::types::MoveActorAbsoluteData;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 18)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct MoveActorAbsolutePacket {
    pub move_data: MoveActorAbsoluteData,
}