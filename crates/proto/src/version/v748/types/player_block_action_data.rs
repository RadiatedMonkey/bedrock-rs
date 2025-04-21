use crate::version::v748::enums::PlayerActionType;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct PlayerBlockActionData {
    pub player_action_type: PlayerActionType,
}