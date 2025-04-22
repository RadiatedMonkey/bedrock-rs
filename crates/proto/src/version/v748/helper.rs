use super::gamepackets::GamePackets;
use crate::helper::ProtoHelper;

pub struct ProtoHelperV748;

impl ProtoHelper for ProtoHelperV748 {
    type GamePacketType = GamePackets;
}
