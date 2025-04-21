use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 160)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct PlayerFogPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub fog_stack: Vec<String>,
}