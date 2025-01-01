use crate::version::v766::enums::CraftingDataEntryType;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct CraftingDataEntry {
    pub crafting_type: CraftingDataEntryType,
}