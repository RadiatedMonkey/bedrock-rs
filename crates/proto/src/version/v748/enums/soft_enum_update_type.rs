use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum SoftEnumUpdateType {
    Add = 0,
    Remove = 1,
    Replace = 2,
}