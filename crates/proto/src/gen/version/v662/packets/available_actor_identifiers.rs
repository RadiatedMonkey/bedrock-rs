use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 119)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct AvailableActorIdentifiersPacket {
    #[nbt]
    pub actor_info_list: nbtx::Value, // TODO: NBT Structure
}