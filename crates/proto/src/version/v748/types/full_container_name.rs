use crate::version::v662::enums::ContainerEnumName;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct FullContainerName {
    pub container_name: ContainerEnumName,
    #[endianness(le)]
    pub dynamic_id: Option<u32>,
}