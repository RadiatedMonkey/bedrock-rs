use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct NetworkPermissions {
    pub server_auth_sound_enabled: bool,
}