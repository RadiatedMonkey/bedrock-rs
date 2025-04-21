use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ScoreboardId {
    #[endianness(var)]
    pub id: i64,
}