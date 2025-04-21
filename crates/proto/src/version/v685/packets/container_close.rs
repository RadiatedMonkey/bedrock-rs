use super::super::enums::ContainerID;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v685::enums::ContainerType;

#[gamepacket(id = 47)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct ContainerClosePacket {
    pub container_id: ContainerID,
    pub container_type: ContainerType,
    pub server_initiated_close: bool,
}