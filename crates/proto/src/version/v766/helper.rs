use crate::helper::ProtoHelper;
use crate::version::v766::gamepackets::GamePackets;

pub struct ProtoHelperV766;

impl ProtoHelper for ProtoHelperV766 {
    type GamePacketType = GamePackets;
}
