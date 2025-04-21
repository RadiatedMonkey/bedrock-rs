use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 10)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct SetTimePacket {
    #[endianness(var)]
    pub time: i32,
}
