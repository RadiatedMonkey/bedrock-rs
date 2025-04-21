use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 193)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct RequestNetworkSettingsPacket {
    #[endianness(be)]
    pub client_network_version: i32,
}