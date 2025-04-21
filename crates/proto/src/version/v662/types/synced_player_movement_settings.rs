use crate::version::v662::enums::ServerAuthMovementMode;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct SyncedPlayerMovementSettings {
    pub authority_mode: ServerAuthMovementMode,
    #[endianness(var)]
    pub rewind_history_size: i32,
    pub server_authoritative_block_breaking: bool,
}