use crate::version::v662::enums::AgentActionType;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 181)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct AgentActionEventPacket {
    pub request_id: String,
    pub action_type: AgentActionType,
    pub response: String,
}