use crate::version::v662::enums::ModalFormCancelReason;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 101)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ModalFormResponsePacket {
    #[endianness(var)]
    pub form_id: u32,
    pub json_response: Option<String>,
    pub form_cancel_reason: Option<ModalFormCancelReason>,
}