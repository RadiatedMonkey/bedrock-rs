use crate::version::v662::enums::{CodeBuilderStorageCategory, CodeBuilderStorageOperation};
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 178)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct CodeBuilderSourcePacket {
    pub operation: CodeBuilderStorageOperation,
    pub category: CodeBuilderStorageCategory,
    pub value: String,
}