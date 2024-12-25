use crate::version::v662::enums::PredictionType;
use vek::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 161)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CorrectPlayerMovePredictionPacket {
    pub prediction_type: PredictionType,
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(le)]
    pub velocity: Vec3<f32>,
    pub on_ground: bool,
    #[endianness(var)]
    pub tick: u64,
}