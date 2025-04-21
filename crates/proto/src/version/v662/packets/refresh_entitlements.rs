use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 305)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct RefreshEntitlementsPacket {}