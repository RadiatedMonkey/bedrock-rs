use super::super::enums::ContainerEnumName;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemStackRequestSlotInfo {
    pub container_net_id: ContainerEnumName,
    pub slot: i8,
    #[endianness(var)]
    pub raw_id: i32,
}