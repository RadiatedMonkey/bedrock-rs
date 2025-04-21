use crate::version::v748::types::FullContainerName;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 317)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ContainerRegistryCleanupPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub removed_containers: Vec<FullContainerName>,
}