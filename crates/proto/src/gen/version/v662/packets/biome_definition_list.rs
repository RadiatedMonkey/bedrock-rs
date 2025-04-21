use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 122)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct BiomeDefinitionListPacket {
    #[nbt]
    pub biome_definition_data: nbtx::Value, // TODO: NBT Structure
}