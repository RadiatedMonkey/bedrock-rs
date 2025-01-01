use crate::version::v662::types::ActorUniqueID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec, Clone, Debug)]
pub struct ClientPixelsListEntry {
    #[endianness(le)]
    pub pixel: u32,
    #[endianness(le)]
    pub index: u16,
}

#[gamepacket(id = 68)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct MapInfoRequestPacket {
    pub map_unique_id: ActorUniqueID,
    #[vec_repr(u32)]
    #[vec_endianness(le)]
    pub client_pixels_list: Vec<ClientPixelsListEntry>,
}