use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 165)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct SyncActorPropertyPacket {
    #[nbt]
    pub property_data: nbtx::Value, // TODO: NBT Structure
}