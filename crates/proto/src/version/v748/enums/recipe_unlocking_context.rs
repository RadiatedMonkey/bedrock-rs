use crate::version::v662::types::RecipeIngredient;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum RecipeUnlockingContext {
    None {
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        unlocking_ingredients: Vec<RecipeIngredient>
    } = 0,
    AlwaysUnlocked = 1,
    PlayerInWater = 2,
    PlayerHasManyItems = 3,
}