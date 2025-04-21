use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct PlayerInputTick {
    #[endianness(var)]
    pub tick: u64,
}