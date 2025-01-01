use bedrockrs_macros::{gamepacket, ProtoCodec};


#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum AuthMovementMode {
    LegacyClientAuthoritativeV1 = 0,
    ClientAuthoritativeV2 = 1,
    ServerAuthoritativeV3 = 2,
}

#[gamepacket(id = 319)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct SetMovementAuthorityPacket {
    pub new_auth_movement_mode: AuthMovementMode,
}