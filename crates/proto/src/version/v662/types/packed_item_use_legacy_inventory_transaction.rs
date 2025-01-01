use crate::version::v662::enums::ItemUseInventoryTransactionType;
use crate::version::v662::types::{
    InventoryTransaction, NetworkBlockPosition, NetworkItemStackDescriptor,
};
use bedrockrs_macros::ProtoCodec;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecLE, ProtoCodecVAR};
use std::io::Cursor;
use vek::Vec3;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ContainerSlotEntry {
    pub container_enum_name: String,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub slots: Vec<i8>,
}

#[derive(Clone, Debug)]
pub struct PackedItemUseLegacyInventoryTransaction {
    id: i32,
    container_slots: Option<Vec<ContainerSlotEntry>>,
    action: InventoryTransaction,
    action_type: ItemUseInventoryTransactionType,
    position: NetworkBlockPosition,
    face: i32,
    slot: i32,
    item: NetworkItemStackDescriptor,
    from_position: Vec3<f32>,
    click_position: Vec3<f32>,
    target_block_id: u32,
}

impl ProtoCodec for PackedItemUseLegacyInventoryTransaction {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        ProtoCodecVAR::proto_serialize(&self.id, stream)?;

        match &self.id {
            0 => {}
            _ => {
                let vec = self.container_slots.as_ref().unwrap();
                let len: u32 = vec.len().try_into()?;
                ProtoCodecVAR::proto_serialize(&len, stream)?;
                for i in vec {
                    i.proto_serialize(stream)?
                }
            }
        }

        self.action.proto_serialize(stream)?;
        self.action_type.proto_serialize(stream)?;
        self.position.proto_serialize(stream)?;
        ProtoCodecVAR::proto_serialize(&self.face, stream)?;
        ProtoCodecVAR::proto_serialize(&self.slot, stream)?;
        self.item.proto_serialize(stream)?;
        ProtoCodecLE::proto_serialize(&self.from_position, stream)?;
        ProtoCodecLE::proto_serialize(&self.click_position, stream)?;
        ProtoCodecVAR::proto_serialize(&self.target_block_id, stream)?;

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let id = <i32 as ProtoCodecVAR>::proto_deserialize(stream)?;
        let container_slots = match id {
            0 => None,
            _ => {
                let len = <u32 as ProtoCodecVAR>::proto_deserialize(stream)?;
                let mut vec = Vec::with_capacity(len.try_into()?);
                for _ in 0..len {
                    vec.push(ContainerSlotEntry::proto_deserialize(stream)?);
                }
                Some(vec)
            }
        };
        let action = InventoryTransaction::proto_deserialize(stream)?;
        let action_type = ItemUseInventoryTransactionType::proto_deserialize(stream)?;
        let position = NetworkBlockPosition::proto_deserialize(stream)?;
        let face = <i32 as ProtoCodecVAR>::proto_deserialize(stream)?;
        let slot = <i32 as ProtoCodecVAR>::proto_deserialize(stream)?;
        let item = NetworkItemStackDescriptor::proto_deserialize(stream)?;
        let from_position = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let click_position = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let target_block_id = <u32 as ProtoCodecVAR>::proto_deserialize(stream)?;

        Ok(Self {
            id,
            container_slots,
            action,
            action_type,
            position,
            face,
            slot,
            item,
            from_position,
            click_position,
            target_block_id,
        })
    }

    fn get_size_prediction(&self) -> usize {
        ProtoCodecVAR::get_size_prediction(&self.id)
            + match &self.id {
            0 => 0,
            _ => {
                let vec = self.container_slots.as_ref().unwrap();
                vec.len() + vec.iter().map(|i| i.get_size_prediction()).sum::<usize>()
            }
        }
            + self.action.get_size_prediction()
            + self.action_type.get_size_prediction()
            + self.position.get_size_prediction()
            + ProtoCodecVAR::get_size_prediction(&self.face)
            + ProtoCodecVAR::get_size_prediction(&self.slot)
            + self.item.get_size_prediction()
            + ProtoCodecLE::get_size_prediction(&self.from_position)
            + ProtoCodecLE::get_size_prediction(&self.click_position)
            + ProtoCodecVAR::get_size_prediction(&self.target_block_id)
    }
}
