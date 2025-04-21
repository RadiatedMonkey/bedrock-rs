use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 129)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ClientCacheStatusPacket {
    pub is_cache_supported: bool,
}