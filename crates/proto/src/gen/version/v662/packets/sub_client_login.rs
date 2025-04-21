use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 94)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct SubClientLoginPacket {
    pub connection_request: String, // TODO: SubClientConnectionRequest diagram, not sure.
}