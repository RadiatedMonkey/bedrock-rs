use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum UIProfile {
    Classic = 0,
    Pocket = 1,
    None = 2,
    Count = 3,
}