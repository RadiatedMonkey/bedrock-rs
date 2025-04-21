use crate::version::v662::types::CameraPresets;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 198)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct CameraPresetsPacket {
    pub camera_presets: CameraPresets,
}