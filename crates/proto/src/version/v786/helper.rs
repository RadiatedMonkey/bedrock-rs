use super::gamepackets::GamePackets;
use crate::helper::ProtoHelper;

pub struct ProtoHelperV786;

impl ProtoHelper for ProtoHelperV786 {
    type GamePacketType = GamePackets;
}
