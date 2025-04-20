use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 312)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct ServerBoundLoadingScreenPacket {
    pub loading_screen_type: LoadingScreenType,
    #[endianness(var)]
    pub loading_screen_id: Option<u32>,
}

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i32)]
#[enum_endianness(var)]
#[repr(i32)]
pub enum LoadingScreenType {
    Unknown = 0,
    StartLoadingScreen = 1,
    EndLoadingScreen = 2,
}
