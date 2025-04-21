use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 130)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct OnScreenTextureAnimationPacket {
    #[endianness(le)]
    pub effect_id: u32,
}