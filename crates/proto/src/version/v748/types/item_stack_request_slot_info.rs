use crate::v748::types::FullContainerName;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemStackRequestSlotInfo {
    pub full_container_name: FullContainerName,
    pub slot: i8,
    #[endianness(var)]
    pub raw_id: i32,
}