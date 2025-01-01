use crate::helper::ProtoHelper;
use crate::version::v662::gamepackets::GamePackets;

pub struct ProtoHelperV662;

impl ProtoHelper for ProtoHelperV662 {
    type GamePacketType = GamePackets;
}
