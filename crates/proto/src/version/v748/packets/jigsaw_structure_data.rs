use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 313)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct JigsawStructureDataPacket {
    #[nbt]
    pub tag: nbtx::Value, // TODO: NBT Structure
}