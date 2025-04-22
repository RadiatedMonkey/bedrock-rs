use super::gamepackets::GamePackets;
use crate::helper::ProtoHelper;

pub struct ProtoHelperV685;

impl ProtoHelper for ProtoHelperV685 {
    type GamePacketType = GamePackets;
}
