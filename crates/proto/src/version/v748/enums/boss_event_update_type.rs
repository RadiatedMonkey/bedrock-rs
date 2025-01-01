use crate::version::v662::types::ActorUniqueID;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u32)]
#[enum_endianness(var)]
#[repr(u32)]
pub enum BossEventUpdateType {
    Add {
        name: String,
        #[endianness(le)]
        health_percent: f32,
        #[endianness(le)]
        darken_screen: u16,
        #[endianness(var)]
        color: u32,
        #[endianness(var)]
        overlay: u32,
    } = 0,
    PlayerAdded {
        player_id: ActorUniqueID,
    } = 1,
    Remove = 2,
    PlayerRemoved {
        player_id: ActorUniqueID,
    } = 3,
    UpdatePercent {
        #[endianness(le)]
        health_percent: f32,
    } = 4,
    UpdateName {
        name: String,
    } = 5,
    UpdateProperties {
        #[endianness(le)]
        darken_screen: u16,
        #[endianness(var)]
        color: u32,
        #[endianness(var)]
        overlay: u32,
    } = 6,
    UpdateStyle {
        #[endianness(var)]
        color: u32,
        #[endianness(var)]
        overlay: u32,
    } = 7,
    Query {
        player_id: ActorUniqueID,
    } = 8,
}