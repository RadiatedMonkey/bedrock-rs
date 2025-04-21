use crate::version::v662::enums::ContainerID;
use crate::version::v662::types::NetworkItemStackDescriptor;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 50)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct InventorySlotPacket {
    pub container_id: ContainerID,
    #[endianness(var)]
    pub slot: u32,
    pub item: NetworkItemStackDescriptor,
}