use crate::version::v748::types::ItemStackResponseSlotInfo;
use bedrockrs_macros::ProtoCodec;
use crate::v748::types::FullContainerName;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemStackResponseContainerInfo {
    pub full_container_name: FullContainerName,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub slots: Vec<ItemStackResponseSlotInfo>
}