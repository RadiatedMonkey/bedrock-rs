use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 38)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct HurtArmorPacket {
    #[endianness(var)]
    pub cause: i32,
    #[endianness(var)]
    pub damage: i32,
    #[endianness(var)]
    pub armor_slots: u64,
}
