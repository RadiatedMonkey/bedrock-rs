use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 315)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ServerBoundDiagnosticsPacket {
    #[endianness(le)]
    pub average_fps: f32,
    #[endianness(le)]
    pub average_server_time: f32,
    #[endianness(le)]
    pub average_client_time: f32,
    #[endianness(le)]
    pub average_begin_frame_time: f32,
    #[endianness(le)]
    pub average_input_time: f32,
    #[endianness(le)]
    pub average_render_time: f32,
    #[endianness(le)]
    pub average_end_frame_time: f32,
    #[endianness(le)]
    pub average_remaining_time_percent: f32,
    #[endianness(le)]
    pub average_unaccounted_time_percent: f32,
}