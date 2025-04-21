use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct PositionTrackingId {
    #[endianness(var)]
    pub value: i32,
}