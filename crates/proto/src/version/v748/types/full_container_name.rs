use crate::version::v662::enums::ContainerName;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct FullContainerName {
    pub container_name: ContainerName,
    #[endianness(le)]
    pub dynamic_id: Option<u32>,
}