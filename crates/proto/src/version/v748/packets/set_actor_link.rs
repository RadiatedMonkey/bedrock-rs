use crate::version::v748::types::ActorLink;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 41)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct SetActorLinkPacket {
    pub link: ActorLink,
}