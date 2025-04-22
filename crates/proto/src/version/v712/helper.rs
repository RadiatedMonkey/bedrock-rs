use super::gamepackets::GamePackets;
use crate::helper::ProtoHelper;

pub struct ProtoHelperV712;

impl ProtoHelper for ProtoHelperV712 {
    type GamePacketType = GamePackets;
}
