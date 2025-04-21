use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i32)]
#[enum_endianness(var)]
#[allow(proto_gen)]
pub enum HudVisibility {
    Hide = 0,
    Reset = 1,
    Count = 2,
}