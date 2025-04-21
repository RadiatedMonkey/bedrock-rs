use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 163)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct FilterTextPacket {
    pub text: String,
    pub from_server: bool,
}