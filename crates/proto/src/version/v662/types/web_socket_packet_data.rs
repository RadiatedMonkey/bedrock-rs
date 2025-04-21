use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct WebSocketPacketData {
    pub web_socket_server_uri: String,
}