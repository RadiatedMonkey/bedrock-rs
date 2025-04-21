use crate::version::v662::types::DimensionDefinitionGroup;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 180)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct DimensionDataPacket {
    pub dimension_definition_group: DimensionDefinitionGroup,
}