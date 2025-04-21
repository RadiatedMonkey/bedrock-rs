use crate::version::v662::enums::ConnectionFailReason;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use std::io::Cursor;

#[gamepacket(id = 5)]
#[derive(Clone, Debug)]
#[allow(proto_gen)]
pub struct DisconnectPacket {
    pub reason: ConnectionFailReason,
    pub messages: Option<DisconnectPacketMessage>,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct DisconnectPacketMessage {
    pub message: String,
    pub filtered_message: String,
}

impl ProtoCodec for DisconnectPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        self.reason.proto_serialize(stream)?;
        bool::proto_serialize(&self.messages.is_none(), stream)?;

        if let Some(ref message) = self.messages {
            message.proto_serialize(stream)?;
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let reason = ConnectionFailReason::proto_deserialize(stream)?;
        let skip_message = bool::proto_deserialize(stream)?;
        let messages = match !skip_message {
            true => Some(DisconnectPacketMessage::proto_deserialize(stream)?),
            false => None,
        };

        Ok(DisconnectPacket { reason, messages })
    }

    fn get_size_prediction(&self) -> usize {
        self.reason.get_size_prediction() + self.messages.get_size_prediction()
    }
}
