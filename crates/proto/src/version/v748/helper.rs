use crate::helper::ProtoHelper;
use crate::version::v748::gamepackets::GamePackets;

pub struct ProtoHelperV748;

impl ProtoHelper for ProtoHelperV748 {
    type GamePacketType = GamePackets;
}
