use crate::version::v662::types::{InventorySource, NetworkItemStackDescriptor};
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct InventoryAction {
    pub source: InventorySource,
    #[endianness(var)]
    pub slot: u32,
    pub from_item_descriptor: NetworkItemStackDescriptor,
    pub to_item_descriptor: NetworkItemStackDescriptor,
}
