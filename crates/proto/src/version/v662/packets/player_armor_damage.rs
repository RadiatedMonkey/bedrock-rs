use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 149)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct PlayerArmorDamagePacket {
    pub slot_bitset: i8,
    #[endianness(var)]
    pub damage: i32,
}