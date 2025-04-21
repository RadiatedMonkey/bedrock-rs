use crate::version::v662::types::PositionTrackingId;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 154)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct PositionTrackingDBClientRequestPacket {
    pub action: Action,
    pub id: PositionTrackingId,
}

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum Action {
    Query = 0
}
