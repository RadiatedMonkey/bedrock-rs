use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 320)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CameraAimAssistPresetsPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub categories: Vec<CategoriesDefinition>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub presets: Vec<PresetDefinition>,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct CategoriesDefinition {
    pub identifier: String,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub categories: Vec<CategoryDefinition>,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct CategoryDefinition {
    pub name: String,
    pub priorities: CategoryPriorities,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct PriorityEntry {
    pub id: String,
    #[endianness(le)]
    pub priority: i32,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct CategoryPriorities {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub entities: Vec<PriorityEntry>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub blocks: Vec<PriorityEntry>,
    #[endianness(le)]
    pub default_entity: Option<i32>,
    #[endianness(le)]
    pub default_block: Option<i32>,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemSettingsEntry {
    pub item_id: String,
    pub category: String,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct PresetDefinition {
    pub identifier: String,
    pub categories: String,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub exclusion_list: Vec<String>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub liquid_targeting_list: Vec<String>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub item_settings: Vec<ItemSettingsEntry>,
    pub default_item_settings: String,
    pub hand_settings: String,
}
