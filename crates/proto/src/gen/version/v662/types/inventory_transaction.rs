use crate::version::v662::types::InventoryAction;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct InventoryTransaction {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub action: Vec<InventoryAction>,
}