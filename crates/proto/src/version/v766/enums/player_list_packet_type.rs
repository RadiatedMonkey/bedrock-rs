use crate::version::v662::enums::BuildPlatform;
use crate::version::v662::types::{ActorUniqueID, SerializedSkin};
use bedrockrs_macros::ProtoCodec;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecVAR};
use std::io::Cursor;
use std::mem::size_of;
use uuid::Uuid;

#[derive(Clone, Debug)]
#[repr(i8)]
pub enum PlayerListPacketType {
    Add {
        add_player_list: Vec<AddPlayerListEntry>,
    } = 0,
    Remove {
        remove_player_list: Vec<Uuid>
    } = 1,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct AddPlayerListEntry {
    pub uuid: Uuid,
    pub target_actor_id: ActorUniqueID,
    pub player_name: String,
    pub xbl_xuid: String,
    pub platform_chat_id: String,
    pub build_platform: BuildPlatform,
    pub serialized_skin: SerializedSkin,
    pub is_teacher: bool,
    pub is_host: bool,
    pub is_sub_client: bool,
    pub is_trusted_skin: bool,
}

impl ProtoCodec for PlayerListPacketType {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        match self {
            PlayerListPacketType::Add { add_player_list } => {
                i8::proto_serialize(&0, stream)?;
                let len: u32 = add_player_list.len().try_into()?;
                <u32 as ProtoCodecVAR>::proto_serialize(&len, stream)?;

                for i in add_player_list {
                    i.uuid.proto_serialize(stream)?;
                    i.target_actor_id.proto_serialize(stream)?;
                    i.player_name.proto_serialize(stream)?;
                    i.xbl_xuid.proto_serialize(stream)?;
                    i.platform_chat_id.proto_serialize(stream)?;
                    i.build_platform.proto_serialize(stream)?;
                    i.serialized_skin.proto_serialize(stream)?;
                    i.is_teacher.proto_serialize(stream)?;
                    i.is_host.proto_serialize(stream)?;
                    i.is_sub_client.proto_serialize(stream)?;
                }

                for i in add_player_list {
                    i.is_trusted_skin.proto_serialize(stream)?;
                }

                Ok(())
            }
            PlayerListPacketType::Remove { remove_player_list } => {
                i8::proto_serialize(&1, stream)?;
                let len = remove_player_list.len().try_into()?;
                <u32 as ProtoCodecVAR>::proto_serialize(&len, stream)?;

                for i in remove_player_list {
                    i.proto_serialize(stream)?
                }

                Ok(())
            }
        }
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let discriminant = i8::proto_deserialize(stream)?;
        let len = <u32 as ProtoCodecVAR>::proto_deserialize(stream)?;
        let value = match discriminant {
            0 => {
                let mut add_player_list = Vec::with_capacity(len.try_into()?);
                for _ in 0..len {
                    add_player_list.push(AddPlayerListEntry {
                        uuid: Uuid::proto_deserialize(stream)?,
                        target_actor_id: ActorUniqueID::proto_deserialize(stream)?,
                        player_name: String::proto_deserialize(stream)?,
                        xbl_xuid: String::proto_deserialize(stream)?,
                        platform_chat_id: String::proto_deserialize(stream)?,
                        build_platform: BuildPlatform::proto_deserialize(stream)?,
                        serialized_skin: SerializedSkin::proto_deserialize(stream)?,
                        is_teacher: bool::proto_deserialize(stream)?,
                        is_host: bool::proto_deserialize(stream)?,
                        is_sub_client: bool::proto_deserialize(stream)?,
                        is_trusted_skin: false,
                    })
                }

                for i in &mut add_player_list {
                    i.is_trusted_skin = bool::proto_deserialize(stream)?
                }

                PlayerListPacketType::Add { add_player_list }
            }
            1 => {
                let mut remove_player_list = Vec::with_capacity(len.try_into()?);
                for _ in 0..len {
                    remove_player_list.push(Uuid::proto_deserialize(stream)?);
                }

                PlayerListPacketType::Remove { remove_player_list }
            }
            _ => { panic!("Unknown discriminant {}", discriminant) }
        };

        Ok(value)
    }

    fn get_size_prediction(&self) -> usize {
        match self {
            PlayerListPacketType::Add { add_player_list } => {
                size_of::<u32>()
                    + add_player_list.iter().map(|i| i.get_size_prediction()).sum::<usize>()
            }
            PlayerListPacketType::Remove { remove_player_list } => {
                size_of::<u32>()
                    + remove_player_list.iter().map(|i| i.get_size_prediction()).sum::<usize>()
            }
        }
    }
}