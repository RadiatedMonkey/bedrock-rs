use crate::version::v662::enums::{ContainerID, ContainerType};
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 47)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ContainerClosePacket {
    pub container_id: ContainerID,
    pub container_type: ContainerType,
    pub server_initiated_close: bool,
}