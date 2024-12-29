use crate::version::v766::types::ActorLink;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 41)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct SetActorLinkPacket {
    pub link: ActorLink,
}