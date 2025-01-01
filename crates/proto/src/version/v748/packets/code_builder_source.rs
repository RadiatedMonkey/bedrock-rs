use crate::version::v662::enums::{CodeBuilderStorageCategory, CodeBuilderStorageOperation};
use crate::version::v748::enums::CodeBuilderExecutionState;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 178)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CodeBuilderSourcePacket {
    pub operation: CodeBuilderStorageOperation,
    pub category: CodeBuilderStorageCategory,
    pub code_status: CodeBuilderExecutionState,
}