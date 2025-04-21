use crate::version::v662::types::{ActorRuntimeID, NetworkItemStackDescriptor};
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 32)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct MobArmorEquipmentPacket {
    pub target_runtime_id: ActorRuntimeID,
    pub head: NetworkItemStackDescriptor,
    pub torso: NetworkItemStackDescriptor,
    pub legs: NetworkItemStackDescriptor,
    pub feet: NetworkItemStackDescriptor,
    pub body: NetworkItemStackDescriptor,
}