use crate::version::v662::types::ActorRuntimeID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 304)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct AgentAnimationPacket {
    pub agent_animation: i8,
    pub runtime_id: ActorRuntimeID,
}