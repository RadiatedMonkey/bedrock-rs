use crate::version::v662::types::{ContainerMixDataEntry, MaterialReducerDataEntry, PotionMixDataEntry};
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v766::types::CraftingDataEntry;

#[gamepacket(id = 52)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CraftingDataPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub crafting_entries: Vec<CraftingDataEntry>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub potion_mixes: Vec<PotionMixDataEntry>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub container_mixes: Vec<ContainerMixDataEntry>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub material_reducers: Vec<MaterialReducerDataEntry>,
    pub clear_recipes: bool,
}
