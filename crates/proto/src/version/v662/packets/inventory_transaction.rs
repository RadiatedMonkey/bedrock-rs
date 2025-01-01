use crate::version::v662::enums::ComplexInventoryTransactionType;
use crate::version::v662::types::InventoryTransaction;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec, Clone, Debug)]
pub struct LegacySetItemSlotsEntry {
    pub container_enum: i8, // TODO: find container enum?
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub slot_vector: Vec<i8>, // TODO: find slot enum? (i8 is Slot)
}

#[gamepacket(id = 30)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct InventoryTransactionPacket {
    #[endianness(var)]
    pub raw_id: i32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub legacy_set_item_slots: Vec<LegacySetItemSlotsEntry>,
    pub transaction_type: ComplexInventoryTransactionType,
    pub transaction: InventoryTransaction,
}