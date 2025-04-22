use super::gamepackets::GamePackets;
use crate::helper::ProtoHelper;

pub struct ProtoHelperV671;

impl ProtoHelper for ProtoHelperV671 {
    type GamePacketType = GamePackets;
}
