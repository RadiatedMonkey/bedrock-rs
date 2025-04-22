use super::gamepackets::GamePackets;
use crate::helper::ProtoHelper;

pub struct ProtoHelperV766;

impl ProtoHelper for ProtoHelperV766 {
    type GamePacketType = GamePackets;
}
