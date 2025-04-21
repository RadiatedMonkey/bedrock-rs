use crate::version::v662::enums::InventorySourceType;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct InventorySource {
    pub source_type: InventorySourceType,
}