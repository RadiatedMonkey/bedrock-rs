use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct PlayerInputTick {
    #[endianness(var)]
    pub tick: u64,
}