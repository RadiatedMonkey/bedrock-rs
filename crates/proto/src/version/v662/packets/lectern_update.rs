use crate::version::v662::types::NetworkBlockPosition;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 125)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct LecternUpdatePacket {
    pub new_page_to_show: i8,
    pub total_pages: i8,
    pub position_of_lectern_to_update: NetworkBlockPosition,
}