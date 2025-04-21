use crate::helper::ProtoHelper;
use super::gamepackets::GamePackets;

pub struct ProtoHelperV685;

impl ProtoHelper for ProtoHelperV685 {
    type GamePacketType = GamePackets;
}
