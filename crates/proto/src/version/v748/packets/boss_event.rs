use crate::version::v662::types::ActorUniqueID;
use crate::version::v748::enums::BossEventUpdateType;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 74)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct BossEventPacket {
    pub target_actor_id: ActorUniqueID,
    pub event_type: BossEventUpdateType,
}