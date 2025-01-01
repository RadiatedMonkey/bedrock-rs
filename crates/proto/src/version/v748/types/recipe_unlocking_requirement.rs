use crate::version::v748::enums::RecipeUnlockingContext;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct RecipeUnlockingRequirement {
    pub unlocking_context: RecipeUnlockingContext,
}