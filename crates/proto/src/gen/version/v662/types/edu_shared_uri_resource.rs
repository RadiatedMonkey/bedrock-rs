use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct EduSharedUriResource {
    pub button_name: String,
    pub link_uri: String,
}