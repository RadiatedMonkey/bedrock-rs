use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value17 {
    pub tags: Struct2,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinecraftTags {
    pub name: String,
    pub value: Value17,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value16 {
    pub name: Struct1,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UsingConvertsTo {
    pub name: String,
    pub value: Value16,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value15 {
    pub can_always_eat: Struct,
    pub nutrition: Struct,
    pub saturation_modifier: Struct4,
    pub using_converts_to: UsingConvertsTo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinecraftFood {
    pub name: String,
    pub value: Value15,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value14 {
    pub max: Struct,
    pub min: Struct,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DamageChance {
    pub name: String,
    pub value: Value14,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value13 {
    pub damage_chance: DamageChance,
    pub max_durability: Struct,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinecraftDurability {
    pub name: String,
    pub value: Value13,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value12 {
    pub value: Struct1,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Struct6 {
    pub name: String,
    pub value: Value12,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value11 {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct States {
    pub name: String,
    pub value: Value11,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value10 {
    pub name: Struct1,
    pub states: States,
    pub tags: Struct1,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub name: String,
    pub value: Value10,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value9 {
    pub block: Block,
    pub speed: Struct,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Struct5 {
    pub name: String,
    pub value: Value9,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DestroySpeeds {
    pub name: String,
    pub value: Vec<Struct5>,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value8 {
    pub destroy_speeds: DestroySpeeds,
    pub use_efficiency: Struct,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinecraftDigger {
    pub name: String,
    pub value: Value8,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Struct4 {
    pub name: String,
    pub value: f64,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value7 {
    pub category: Struct1,
    pub duration: Struct4,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinecraftCooldown {
    pub name: String,
    pub value: Value7,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value6 {
    pub num_viewable_slots: Struct,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinecraftBundleInteraction {
    pub name: String,
    pub value: Value6,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value5 {
    pub value: Struct,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Struct3 {
    pub name: String,
    pub value: Value5,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Struct2 {
    pub name: String,
    pub value: Vec<Struct1>,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value4 {
    pub default: Struct1,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Textures {
    pub name: String,
    pub value: Value4,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value3 {
    pub textures: Textures,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinecraftIcon {
    pub name: String,
    pub value: Value3,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Struct1 {
    pub name: String,
    pub value: String,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Struct {
    pub name: String,
    pub value: i64,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value2 {
    pub allow_off_hand: Struct,
    pub can_destroy_in_creative: Struct,
    pub creative_category: Struct,
    pub creative_group: Struct1,
    pub damage: Struct,
    pub enchantable_slot: Struct1,
    pub enchantable_value: Struct,
    pub foil: Struct,
    pub frame_count: Struct,
    pub hand_equipped: Struct,
    pub hover_text_color: Struct1,
    pub liquid_clipped: Struct,
    pub max_stack_size: Struct,
    #[serde(rename = "minecraft:icon")]
    pub minecraft_icon: MinecraftIcon,
    pub mining_speed: Struct,
    pub should_despawn: Struct,
    pub stacked_by_data: Struct,
    pub use_animation: Struct,
    pub use_duration: Struct,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemProperties {
    pub name: String,
    pub value: Value2,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value1 {
    pub item_properties: ItemProperties,
    pub item_tags: Struct2,
    #[serde(rename = "minecraft:allow_off_hand")]
    pub minecraft_allow_off_hand: Struct3,
    #[serde(rename = "minecraft:bundle_interaction")]
    pub minecraft_bundle_interaction: MinecraftBundleInteraction,
    #[serde(rename = "minecraft:can_destroy_in_creative")]
    pub minecraft_can_destroy_in_creative: Struct3,
    #[serde(rename = "minecraft:cooldown")]
    pub minecraft_cooldown: MinecraftCooldown,
    #[serde(rename = "minecraft:damage")]
    pub minecraft_damage: Struct3,
    #[serde(rename = "minecraft:digger")]
    pub minecraft_digger: MinecraftDigger,
    #[serde(rename = "minecraft:display_name")]
    pub minecraft_display_name: Struct6,
    #[serde(rename = "minecraft:durability")]
    pub minecraft_durability: MinecraftDurability,
    #[serde(rename = "minecraft:food")]
    pub minecraft_food: MinecraftFood,
    #[serde(rename = "minecraft:hand_equipped")]
    pub minecraft_hand_equipped: Struct3,
    #[serde(rename = "minecraft:hover_text_color")]
    pub minecraft_hover_text_color: Struct6,
    #[serde(rename = "minecraft:rarity")]
    pub minecraft_rarity: Struct6,
    #[serde(rename = "minecraft:tags")]
    pub minecraft_tags: MinecraftTags,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Components {
    pub name: String,
    pub value: Value1,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value {
    pub components: Components,
    pub id: Struct,
    pub name: Struct1,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemComponentDataNBT {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: i64,
    pub value: Value,
}