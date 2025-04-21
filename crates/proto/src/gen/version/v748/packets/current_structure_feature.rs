use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 314)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct CurrentStructureFeaturePacket {
    pub current_structure_feature: String,
}