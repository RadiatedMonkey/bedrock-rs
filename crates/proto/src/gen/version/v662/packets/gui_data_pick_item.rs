use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 54)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct GuiDataPickItemPacket {
    pub item_name: String,
    pub item_effect_name: String,
    #[endianness(le)]
    pub slot: i32,
}