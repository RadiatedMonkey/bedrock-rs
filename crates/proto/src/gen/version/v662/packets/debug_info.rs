use crate::version::v662::types::ActorUniqueID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 155)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct DebugInfoPacket {
    pub actor_id: ActorUniqueID,
    pub data: String,
}