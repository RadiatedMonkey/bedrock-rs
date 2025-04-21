use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 70)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ChunkRadiusUpdatedPacket {
    #[endianness(var)]
    pub chunk_radius: i32,
}