use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 20)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct PassengerJumpPacket {
    #[endianness(var)]
    pub jump_scale: i32,
}