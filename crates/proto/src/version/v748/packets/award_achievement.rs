use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 309)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct AwardAchievementPacket {
    #[endianness(le)]
    pub achievement_id: i32,
}