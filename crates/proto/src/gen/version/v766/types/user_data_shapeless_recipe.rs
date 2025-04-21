use crate::version::v662::types::{NetworkItemInstanceDescriptor, RecipeIngredient};
use crate::version::v748::types::RecipeUnlockingRequirement;
use bedrockrs_macros::ProtoCodec;
use uuid::Uuid;

#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct UserDataShapelessRecipe {
    pub recipe_unique_id: String,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub ingredient_list: Vec<RecipeIngredient>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub production_list: Vec<NetworkItemInstanceDescriptor>,
    pub recipe_id: Uuid,
    pub recipe_tag: String,
    #[endianness(var)]
    pub priority: i32,
    pub unlocking_requirement: RecipeUnlockingRequirement,
}