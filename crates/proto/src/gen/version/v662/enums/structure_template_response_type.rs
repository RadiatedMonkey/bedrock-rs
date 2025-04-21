use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
#[allow(proto_gen)]
pub enum StructureTemplateResponseType {
    None = 0,
    Export = 1,
    Query = 2,
    Import = 3,
}