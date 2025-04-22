use super::super::enums::ContainerID;
use super::super::types::NetworkItemStackDescriptor;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_proto_core::ProtoCodec;

#[gamepacket(id = 50)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct InventorySlotPacket {
    #[endianness(var)]
    pub container_id: i32,
    #[endianness(var)]
    pub slot: u32,
    #[endianness(var)]
    pub container_name_dynamic_id: i32,
    pub item: NetworkItemStackDescriptor,
}