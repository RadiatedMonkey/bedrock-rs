use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 59)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct SetCommandsEnabledPacket {
    pub commands_enabled: bool,
}
