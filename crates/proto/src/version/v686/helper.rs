use super::gamepackets::GamePackets;
use crate::helper::ProtoHelper;

pub struct ProtoHelperV686;

impl ProtoHelper for ProtoHelperV686 {
    type GamePacketType = GamePackets;
}
