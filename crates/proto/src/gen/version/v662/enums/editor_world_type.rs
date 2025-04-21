use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i32)]
#[enum_endianness(var)]
#[repr(i32)]
#[allow(proto_gen)]
pub enum EditorWorldType {
    NonEditor = 0,
    EditorProject = 1,
    EditorTestLevel = 2,
}