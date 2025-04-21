use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 313)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct JigsawStructureDataPacket {
    #[nbt]
    pub tag: nbtx::Value, // TODO: NBT Structure
}