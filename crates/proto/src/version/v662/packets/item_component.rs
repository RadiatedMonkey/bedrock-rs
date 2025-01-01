use serde::{Deserialize, Serialize};
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v662::nbt::ItemComponentDataNBT;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemsEntry {
    pub component_item_name: String,
    #[nbt]
    pub component_data: ItemComponentDataNBT, // TODO: NBT Structure
}

#[gamepacket(id = 162)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemComponentPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub items: Vec<ItemsEntry>,
}