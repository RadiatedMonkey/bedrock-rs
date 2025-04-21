use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 42)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct SetHealthPacket {
    #[endianness(var)]
    pub health: i32,
}
