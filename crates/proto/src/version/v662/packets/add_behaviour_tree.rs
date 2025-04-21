use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 89)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct AddBehaviourTreePacket {
    pub json_behaviour_tree_structure: String,
}