use super::super::enums::ItemDescriptorInternalType;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct RecipeIngredient {
    pub internal_type: ItemDescriptorInternalType,
    #[endianness(var)]
    pub stack_size: i32,
}