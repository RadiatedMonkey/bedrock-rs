use super::gamepackets::GamePackets;
use crate::helper::ProtoHelper;

pub struct ProtoHelperV662;

impl ProtoHelper for ProtoHelperV662 {
    type GamePacketType = GamePackets;
}
