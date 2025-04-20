use crate::version::v662::enums::{
    ClientPlayMode, InputMode, ItemStackRequestActionType, NewInteractionModel,
    TextProcessingEventOrigin,
};
use crate::version::v662::types::{
    ActorUniqueID,
};
use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecLE, ProtoCodecVAR};
use std::io::Cursor;
use vek::{Vec2, Vec3};
use crate::version::v748::types::PlayerBlockActions;
use crate::version::v748::types::{ItemStackRequestSlotInfo, PackedItemUseLegacyInventoryTransaction};

#[gamepacket(id = 144)]
#[derive(Clone, Debug)]
pub struct PlayerAuthInputPacket {
    pub player_rotation: Vec2<f32>,
    pub player_position: Vec3<f32>,
    pub move_vector: Vec3<f32>,
    pub player_head_rotation: f32,
    pub input_data: u64,
    pub input_mode: InputMode,
    pub play_mode: ClientPlayMode,
    pub new_interaction_model: NewInteractionModel,
    pub interact_rotation: Vec3<f32>,
    pub client_tick: u64,
    pub velocity: Vec3<f32>,
    pub item_use_transaction: Option<PackedItemUseLegacyInventoryTransaction>, // If input_data has PlayerAuthInputPacket::InputData::PerformItemInteraction set.
    pub item_stack_request: Option<PerformItemStackRequestData>, // If input data has PlayerAuthInputPacket::InputData::PerformItemStackRequest set.
    pub player_block_actions: Option<PlayerBlockActions>, // If input data has PlayerAuthInputPacket::InputData::PerformBlockActions set.
    pub client_predicted_vehicle: Option<ClientPredictedVehicleData>, // If input data has PlayerAuthInputPacket::InputData::IsInClientPredictedVehicle set.
    pub analog_move_vector: Vec2<f32>,
    pub camera_orientation: Vec3<f32>,
}

// VERIFY: ProtoCodec impl

pub struct PlayerAuthInputFlags;

impl PlayerAuthInputFlags {
    pub const ASCEND: u64 = 1 << 0;
    pub const DESCEND: u64 = 1 << 1;
    #[deprecated]
    pub const NORTH_JUMP: u64 = 1 << 2;
    pub const JUMP_DOWN: u64 = 1 << 3;
    pub const SPRINT_DOWN: u64 = 1 << 4;
    pub const CHANGE_HEIGHT: u64 = 1 << 5;
    pub const JUMPING: u64 = 1 << 6;
    pub const AUTO_JUMPING_IN_WATER: u64 = 1 << 7;
    pub const SNEAKING: u64 = 1 << 8;
    pub const SNEAK_DOWN: u64 = 1 << 9;
    pub const UP: u64 = 1 << 10;
    pub const DOWN: u64 = 1 << 11;
    pub const LEFT: u64 = 1 << 12;
    pub const RIGHT: u64 = 1 << 13;
    pub const UP_LEFT: u64 = 1 << 14;
    pub const UP_RIGHT: u64 = 1 << 15;
    pub const WANT_UP: u64 = 1 << 16;
    pub const WANT_DOWN: u64 = 1 << 17;
    pub const WANT_DOWN_SLOW: u64 = 1 << 18;
    pub const WANT_UP_SLOW: u64 = 1 << 19;
    pub const SPRINTING: u64 = 1 << 20;
    pub const ASCEND_BLOCK: u64 = 1 << 21;
    pub const DESCEND_BLOCK: u64 = 1 << 22;
    pub const SNEAK_TOGGLE_DOWN: u64 = 1 << 23;
    pub const PERSIST_SNEAK: u64 = 1 << 24;
    pub const START_SPRINTING: u64 = 1 << 25;
    pub const STOP_SPRINTING: u64 = 1 << 26;
    pub const START_SNEAKING: u64 = 1 << 27;
    pub const STOP_SNEAKING: u64 = 1 << 28;
    pub const START_SWIMMING: u64 = 1 << 29;
    pub const STOP_SWIMMING: u64 = 1 << 30;
    pub const START_JUMPING: u64 = 1 << 31;
    pub const START_GLIDING: u64 = 1 << 32;
    pub const STOP_GLIDING: u64 = 1 << 33;
    pub const PERFORM_ITEM_INTERACTION: u64 = 1 << 34;
    pub const PERFORM_BLOCK_ACTIONS: u64 = 1 << 35;
    pub const PERFORM_ITEM_STACK_REQUEST: u64 = 1 << 36;
    pub const HANDLE_TELEPORT: u64 = 1 << 37;
    pub const EMOTING: u64 = 1 << 38;
    pub const MISSED_SWING: u64 = 1 << 39;
    pub const START_CRAWLING: u64 = 1 << 40;
    pub const STOP_CRAWLING: u64 = 1 << 41;
    pub const START_FLYING: u64 = 1 << 42;
    pub const STOP_FLYING: u64 = 1 << 43;
    pub const RECEIVED_SERVER_DATA: u64 = 1 << 44;
    pub const IS_IN_CLIENT_PREDICTED_VEHICLE: u64 = 1 << 45;
    pub const PADDLE_LEFT: u64 = 1 << 46;
    pub const PADDLE_RIGHT: u64 = 1 << 47;
    pub const BLOCK_BREAKING_DELAY_ENABLED: u64 = 1 << 48;
    pub const HORIZONTAL_COLLISION: u64 = 1 << 49;
    pub const VERTICAL_COLLISION: u64 = 1 << 50;
    pub const DOWN_LEFT: u64 = 1 << 51;
    pub const DOWN_RIGHT: u64 = 1 << 52;
    pub const START_USING_ITEM: u64 = 1 << 53;
    pub const IS_CAMERA_RELATIVE_MOVEMENT_ENABLED: u64 = 1 << 54;
    pub const IS_ROT_CONTROLLED_BY_MOVE_DIRECTION: u64 = 1 << 55;
    pub const START_SPIN_ATTACK: u64 = 1 << 56;
    pub const STOP_SPIN_ATTACK: u64 = 1 << 57;
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct PerformItemStackRequestData {
    #[endianness(var)]
    pub client_request_id: u32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub actions: Vec<ActionsEntry>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub strings_to_filter: Vec<String>,
    pub strings_to_filter_origin: TextProcessingEventOrigin,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct ClientPredictedVehicleData {
    #[endianness(le)]
    pub vehicle_rotation: Vec2<f32>,
    pub client_predicted_vehicle: ActorUniqueID,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct ActionsEntry {
    pub action_type: ItemStackRequestActionType,
    pub amount: i8,
    pub source: ItemStackRequestSlotInfo,
    pub destination: ItemStackRequestSlotInfo,
}

impl ProtoCodec for PlayerAuthInputPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        <Vec2<f32> as ProtoCodecLE>::proto_serialize(&self.player_rotation, stream)?;
        <Vec3<f32> as ProtoCodecLE>::proto_serialize(&self.player_position, stream)?;
        <Vec3<f32> as ProtoCodecLE>::proto_serialize(&self.move_vector, stream)?;
        <f32 as ProtoCodecLE>::proto_serialize(&self.player_head_rotation, stream)?;
        <u64 as ProtoCodecVAR>::proto_serialize(&self.input_data, stream)?;
        <InputMode as ProtoCodec>::proto_serialize(&self.input_mode, stream)?;
        <ClientPlayMode as ProtoCodec>::proto_serialize(&self.play_mode, stream)?;
        <NewInteractionModel as ProtoCodec>::proto_serialize(&self.new_interaction_model, stream)?;
        <Vec3<f32> as ProtoCodecLE>::proto_serialize(&self.interact_rotation, stream)?;
        <u64 as ProtoCodecVAR>::proto_serialize(&self.client_tick, stream)?;
        <Vec3<f32> as ProtoCodecLE>::proto_serialize(&self.velocity, stream)?;
        if &self.input_data & PlayerAuthInputFlags::PERFORM_ITEM_INTERACTION != 0 {
            <PackedItemUseLegacyInventoryTransaction as ProtoCodec>::proto_serialize(
                &self.item_use_transaction.as_ref().unwrap(),
                stream,
            )?;
        }
        if &self.input_data & PlayerAuthInputFlags::PERFORM_ITEM_STACK_REQUEST != 0 {
            <PerformItemStackRequestData as ProtoCodec>::proto_serialize(
                &self.item_stack_request.as_ref().unwrap(),
                stream,
            )?;
        }
        if &self.input_data & PlayerAuthInputFlags::PERFORM_BLOCK_ACTIONS != 0 {
            <PlayerBlockActions as ProtoCodec>::proto_serialize(
                &self.player_block_actions.as_ref().unwrap(),
                stream,
            )?;
        }
        if &self.input_data & PlayerAuthInputFlags::IS_IN_CLIENT_PREDICTED_VEHICLE != 0 {
            <ClientPredictedVehicleData as ProtoCodec>::proto_serialize(
                &self.client_predicted_vehicle.as_ref().unwrap(),
                stream,
            )?;
        }
        <Vec2<f32> as ProtoCodecLE>::proto_serialize(&self.analog_move_vector, stream)?;
        <Vec3<f32> as ProtoCodecLE>::proto_serialize(&self.camera_orientation, stream)?;

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let player_rotation = <Vec2<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let player_position = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let move_vector = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let player_head_rotation = <f32 as ProtoCodecLE>::proto_deserialize(stream)?;
        let input_data = <u64 as ProtoCodecVAR>::proto_deserialize(stream)?;
        let input_mode = <InputMode as ProtoCodec>::proto_deserialize(stream)?;
        let play_mode = <ClientPlayMode as ProtoCodec>::proto_deserialize(stream)?;
        let new_interaction_model = <NewInteractionModel as ProtoCodec>::proto_deserialize(stream)?;
        let interact_rotation = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let client_tick = <u64 as ProtoCodecVAR>::proto_deserialize(stream)?;
        let velocity = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let item_use_transaction = match &input_data
            & PlayerAuthInputFlags::PERFORM_ITEM_INTERACTION
            != 0
        {
            true => Some(
                <PackedItemUseLegacyInventoryTransaction as ProtoCodec>::proto_deserialize(stream)?,
            ),
            false => None,
        };
        let item_stack_request = match &input_data
            & PlayerAuthInputFlags::PERFORM_ITEM_STACK_REQUEST
            != 0
        {
            true => Some(<PerformItemStackRequestData as ProtoCodec>::proto_deserialize(stream)?),
            false => None,
        };
        let player_block_actions =
            match &input_data & PlayerAuthInputFlags::PERFORM_BLOCK_ACTIONS != 0 {
                true => Some(<PlayerBlockActions as ProtoCodec>::proto_deserialize(
                    stream,
                )?),
                false => None,
            };
        let client_predicted_vehicle = match &input_data
            & PlayerAuthInputFlags::IS_IN_CLIENT_PREDICTED_VEHICLE
            != 0
        {
            true => Some(<ClientPredictedVehicleData as ProtoCodec>::proto_deserialize(stream)?),
            false => None,
        };
        let analog_move_vector = <Vec2<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let camera_orientation = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;

        Ok(Self {
            player_rotation,
            player_position,
            move_vector,
            player_head_rotation,
            input_data,
            input_mode,
            play_mode,
            new_interaction_model,
            interact_rotation,
            client_tick,
            velocity,
            item_use_transaction,
            item_stack_request,
            player_block_actions,
            client_predicted_vehicle,
            analog_move_vector,
            camera_orientation,
        })
    }

    fn get_size_prediction(&self) -> usize {
        ProtoCodecLE::get_size_prediction(&self.player_rotation)
            + ProtoCodecLE::get_size_prediction(&self.player_position)
            + ProtoCodecLE::get_size_prediction(&self.move_vector)
            + ProtoCodecLE::get_size_prediction(&self.player_head_rotation)
            + ProtoCodecVAR::get_size_prediction(&self.input_data)
            + self.input_mode.get_size_prediction()
            + self.play_mode.get_size_prediction()
            + self.new_interaction_model.get_size_prediction()
            + ProtoCodecLE::get_size_prediction(&self.interact_rotation)
            + ProtoCodecVAR::get_size_prediction(&self.client_tick)
            + ProtoCodecLE::get_size_prediction(&self.velocity)
            + match &self.input_data & PlayerAuthInputFlags::PERFORM_ITEM_INTERACTION != 0 {
            true => self.item_use_transaction.get_size_prediction(),
            false => 0,
        }
            + match &self.input_data & PlayerAuthInputFlags::PERFORM_ITEM_STACK_REQUEST != 0 {
            true => self.item_stack_request.get_size_prediction(),
            false => 0,
        }
            + match &self.input_data & PlayerAuthInputFlags::PERFORM_BLOCK_ACTIONS != 0 {
            true => self.player_block_actions.get_size_prediction(),
            false => 0,
        }
            + match &self.input_data & PlayerAuthInputFlags::IS_IN_CLIENT_PREDICTED_VEHICLE != 0
        {
            true => self.client_predicted_vehicle.get_size_prediction(),
            false => 0,
        }
            + ProtoCodecLE::get_size_prediction(&self.analog_move_vector)
            + ProtoCodecLE::get_size_prediction(&self.camera_orientation)
    }
}
