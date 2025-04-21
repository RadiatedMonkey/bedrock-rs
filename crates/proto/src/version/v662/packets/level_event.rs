use crate::version::v662::enums::LevelEvent;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use vek::Vec3;

#[gamepacket(id = 25)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct LevelEventPacket {
    pub event_id: LevelEvent,
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(var)]
    pub data: i32,
}