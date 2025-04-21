use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 100)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ModalFormRequestPacket {
    #[endianness(var)]
    pub form_id: u32,
    pub form_ui_json: String,
}