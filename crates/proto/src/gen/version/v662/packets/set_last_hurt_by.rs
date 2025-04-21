use crate::version::v662::enums::ActorType;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 96)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct SetLastHurtByPacket {
    pub last_hurt_by: ActorType,
}