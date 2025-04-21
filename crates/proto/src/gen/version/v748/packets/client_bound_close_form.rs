use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 310)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ClientBoundCloseFormPacket {}