use super::super::enums::PredictionType;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use vek::Vec3;

#[gamepacket(id = 161)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CorrectPlayerMovePredictionPacket {
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(le)]
    pub velocity: Vec3<f32>,
    pub on_ground: bool,
    #[endianness(var)]
    pub tick: u64,
    pub prediction_type: PredictionType,
}