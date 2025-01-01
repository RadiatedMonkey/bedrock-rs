use crate::version::v662::enums::{BuildPlatform, GameType};
use crate::version::v662::types::{ActorRuntimeID, NetworkItemStackDescriptor, PropertySyncData, SerializedAbilitiesData};
use vek::{Vec2, Vec3};
use bedrockrs_macros::{gamepacket, ProtoCodec};
use uuid::Uuid;
use crate::version::v748::types::ActorLink;
use crate::version::v748::types::DataItem;

#[gamepacket(id = 12)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct AddPlayerPacket {
    pub uuid: Uuid,
    pub player_name: String,
    pub target_runtime_id: ActorRuntimeID,
    pub platform_chat_id: String,
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(le)]
    pub velocity: Vec3<f32>,
    #[endianness(le)]
    pub rotation: Vec2<f32>,
    #[endianness(le)]
    pub y_head_rotation: f32,
    pub carried_item: NetworkItemStackDescriptor,
    pub player_game_type: GameType,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub entity_data: Vec<DataItem>, // VERIFY: vec_repr & vec_endianness
    pub synced_properties: PropertySyncData,
    pub abilities_data: SerializedAbilitiesData,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub actor_links: Vec<ActorLink>,
    pub device_id: String,
    pub build_platform: BuildPlatform,
}