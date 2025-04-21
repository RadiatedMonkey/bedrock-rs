use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i32)]
#[enum_endianness(var)]
#[repr(i32)]
#[allow(proto_gen)]
pub enum InventoryLayout {
    None = 0,
    Survival = 1,
    RecipeBook = 2,
    Creative = 3,
    Count = 4,
}