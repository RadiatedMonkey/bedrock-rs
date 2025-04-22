use super::gamepackets::GamePackets;
use crate::helper::ProtoHelper;

pub struct ProtoHelperV776;

impl ProtoHelper for ProtoHelperV776 {
    type GamePacketType = GamePackets;
}
