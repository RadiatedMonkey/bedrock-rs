use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 102)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ServerSettingsRequestPacket {}