use super::super::enums::ContainerID;
use super::super::types::NetworkItemStackDescriptor;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 50)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct InventorySlotPacket {
    pub container_id: ContainerID,
    #[endianness(var)]
    pub slot: u32,
    pub item: NetworkItemStackDescriptor,
}