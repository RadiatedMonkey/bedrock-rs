use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 186)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ToastRequestPacket {
    pub title: String,
    pub content: String,
}