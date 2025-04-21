use crate::version::v662::types::ActorUniqueID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 14)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct RemoveActorPacket {
    pub target_actor_id: ActorUniqueID,
}
