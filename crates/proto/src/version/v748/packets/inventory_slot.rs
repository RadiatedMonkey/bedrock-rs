use crate::v748::types::FullContainerName;
use crate::version::v662::enums::ContainerID;
use crate::version::v662::types::NetworkItemStackDescriptor;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 50)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct InventorySlotPacket {
    pub container_id: ContainerID,
    #[endianness(var)]
    pub slot: u32,
    pub full_container_name: FullContainerName,
    pub storage_item: NetworkItemStackDescriptor,
    pub item: NetworkItemStackDescriptor,
}