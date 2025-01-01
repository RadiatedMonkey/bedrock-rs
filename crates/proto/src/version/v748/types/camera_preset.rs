use bedrockrs_macros::ProtoCodec;
use vek::{Vec2, Vec3};

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum AudioListener {
    Camera = 0,
    Player = 1,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct CameraPreset {
    pub name: String,
    pub inherit_from: String,
    #[endianness(le)]
    pub pos_x: Option<f32>,
    #[endianness(le)]
    pub pos_y: Option<f32>,
    #[endianness(le)]
    pub pos_z: Option<f32>,
    #[endianness(le)]
    pub rot_x: Option<f32>,
    #[endianness(le)]
    pub rot_y: Option<f32>,
    #[endianness(le)]
    pub rotation_speed: Option<f32>,
    pub snap_to_target: Option<bool>,
    #[endianness(le)]
    pub horizontal_rotation_limit: Option<Vec2<f32>>,
    #[endianness(le)]
    pub vertical_rotation_limit: Option<Vec2<f32>>,
    pub continue_targeting: Option<bool>,
    #[endianness(le)]
    pub view_offset: Option<Vec2<f32>>,
    #[endianness(le)]
    pub entity_offset: Option<Vec3<f32>>,
    #[endianness(le)]
    pub radius: Option<f32>,
    pub listener: Option<AudioListener>,
    pub player_effects: Option<bool>,
    pub align_target_and_camera_forward: Option<bool>,
}