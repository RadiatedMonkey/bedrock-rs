use crate::version::v662::types::SerializedAbilitiesData;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 187)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct UpdateAbilitiesPacket {
    pub data: SerializedAbilitiesData,
}