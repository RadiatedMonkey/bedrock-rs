use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 106)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct RemoveObjectivePacket {
    pub objective_name: String,
}