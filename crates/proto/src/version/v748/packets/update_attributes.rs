use crate::version::v662::enums::{AttributeModifierOperation, AttributeOperands};
use crate::version::v662::types::ActorRuntimeID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec, Clone, Debug)]
pub struct AttributeModifier {
    pub id: String,
    pub name: String,
    #[endianness(le)]
    pub amount: f32,
    pub operation: AttributeModifierOperation,
    pub operand: AttributeOperands,
    pub is_serializable: bool,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct AttributeData {
    #[endianness(le)]
    pub min_value: f32,
    #[endianness(le)]
    pub max_value: f32,
    #[endianness(le)]
    pub current_value: f32,
    #[endianness(le)]
    pub default_min_value: f32,
    #[endianness(le)]
    pub default_max_value: f32,
    #[endianness(le)]
    pub default_value: f32,
    pub name: String,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub modifiers: Vec<AttributeModifier>,
}

#[gamepacket(id = 29)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct UpdateAttributesPacket {
    pub target_runtime_id: ActorRuntimeID,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub attribute_list: Vec<AttributeData>,
    #[endianness(var)]
    pub tick: u64,
}