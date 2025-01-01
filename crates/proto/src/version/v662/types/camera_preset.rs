use bedrockrs_macros::ProtoCodec;

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
    pub listener: Option<AudioListener>,
    pub player_effects: Option<bool>,
}