use crate::v662::types::ActorRuntimeID;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 318)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct MovementEffectPacket {
    pub target_runtime_id: ActorRuntimeID,
    #[endianness(var)]
    pub effect_id: i32,
    #[endianness(var)]
    pub effect_duration: i32,
    #[endianness(var)]
    pub tick: i64,
}