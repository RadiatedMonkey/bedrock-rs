use std::collections::BTreeMap;
use std::io::{Cursor, Write};

use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use serde_json::Value;
use varint_rs::VarintWriter;

#[gamepacket(id = 3)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct HandshakeServerToClientPacket {
    pub jwt: String,
}
