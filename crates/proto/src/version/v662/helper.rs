use crate::helper::ProtoHelper;
use super::gamepackets::GamePackets;

pub struct ProtoHelperV662;

impl ProtoHelper for ProtoHelperV662 {
    type GamePacketType = GamePackets;
}
