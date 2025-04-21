use crate::version::v662::types::EducationLevelSettings;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 137)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct EducationSettingsPacket {
    pub education_level_settings: EducationLevelSettings,
}