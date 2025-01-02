use bedrockrs_macros::ProtoCodec;
use serde_repr::Deserialize_repr;

#[derive(ProtoCodec, Deserialize_repr, Clone, Debug)]
#[enum_repr(u32)]
#[enum_endianness(var)]
#[repr(u32)]
pub enum InputMode {
    Undefined = 0,
    Mouse = 1,
    Touch = 2,
    GamePad = 3,
    MotionController = 4,
    Count = 5,
}
