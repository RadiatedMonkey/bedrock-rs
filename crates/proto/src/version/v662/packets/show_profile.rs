use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 104)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ShowProfilePacket {
    pub player_xuid: String,
}