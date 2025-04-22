use super::gamepackets::GamePackets;
use crate::helper::ProtoHelper;

pub struct ProtoHelperV729;

impl ProtoHelper for ProtoHelperV729 {
    type GamePacketType = GamePackets;
}
