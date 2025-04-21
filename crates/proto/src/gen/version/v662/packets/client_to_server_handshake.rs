use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 4)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ClientToServerHandshakePacket {}