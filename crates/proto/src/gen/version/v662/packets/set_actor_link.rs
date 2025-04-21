use crate::version::v662::types::ActorLink;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 41)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct SetActorLinkPacket {
    pub link: ActorLink,
}