use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 140)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct SettingsCommandPacket {
    pub command: String,
    pub suppress_output: bool,
}