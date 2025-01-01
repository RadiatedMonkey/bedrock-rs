use bedrockrs_macros::{gamepacket, ProtoCodec};
use vek::Vec3;

#[gamepacket(id = 18)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ServerPlayerPostMovePositionPacket {
    #[endianness(le)]
    pub pos: Vec3<f32>,
}
