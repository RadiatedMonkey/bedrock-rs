use crate::version::v662::types::{
    ActorUniqueID, BlockPos, MapDecoration, MapItemTrackedActorUniqueID,
};
use bedrockrs_macros::gamepacket;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecVAR};
use std::io::Cursor;
use std::mem::size_of;

#[gamepacket(id = 67)]
#[derive(Clone, Debug)]
#[allow(proto_gen)]
pub struct ClientBoundMapItemDataPacket {
    pub map_id: ActorUniqueID,
    pub type_flags: u32,
    pub dimension: i8,
    pub is_locked: bool,
    pub map_origin: BlockPos,
    pub map_id_list: Option<Vec<ActorUniqueID>>,
    pub scale: Option<i8>,
    pub actor_ids: Option<Vec<MapItemTrackedActorUniqueID>>,
    pub decoration_list: Option<Vec<MapDecoration>>,
    pub texture_width: Option<i32>,
    pub texture_height: Option<i32>,
    pub x_tex_coordinate: Option<i32>,
    pub y_tex_coordinate: Option<i32>,
    pub pixels: Option<Vec<u32>>,
}

#[repr(u32)]
pub enum Type {
    Invalid = 0x0,
    TextureUpdate = 0x2,
    DecorationUpdate = 0x4,
    Creation = 0x8,
}

impl ProtoCodec for ClientBoundMapItemDataPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        self.map_id.proto_serialize(stream)?;
        <u32 as ProtoCodecVAR>::proto_serialize(&self.type_flags, stream)?;
        self.dimension.proto_serialize(stream)?;
        self.is_locked.proto_serialize(stream)?;
        self.map_origin.proto_serialize(stream)?;

        if self.type_flags & Type::Creation as u32 != 0 {
            let vec = self.map_id_list.as_ref().unwrap();
            let len: u32 = vec.len().try_into()?;
            <u32 as ProtoCodecVAR>::proto_serialize(&len, stream)?;
            for i in vec {
                i.proto_serialize(stream)?
            }
        }

        if self.type_flags
            & (Type::DecorationUpdate as u32 | Type::TextureUpdate as u32 | Type::Creation as u32)
            != 0
        {
            self.scale.as_ref().unwrap().proto_serialize(stream)?;
        }

        if self.type_flags & Type::DecorationUpdate as u32 != 0 {
            {
                let vec = self.actor_ids.as_ref().unwrap();
                let len: u32 = vec.len().try_into()?;
                <u32 as ProtoCodecVAR>::proto_serialize(&len, stream)?;
                for i in vec {
                    i.proto_serialize(stream)?
                }
            }
            {
                let vec = self.decoration_list.as_ref().unwrap();
                let len: u32 = vec.len().try_into()?;
                <u32 as ProtoCodecVAR>::proto_serialize(&len, stream)?;
                for i in vec {
                    i.proto_serialize(stream)?
                }
            }
        }

        if self.type_flags & Type::TextureUpdate as u32 != 0 {
            <i32 as ProtoCodecVAR>::proto_serialize(self.texture_width.as_ref().unwrap(), stream)?;
            <i32 as ProtoCodecVAR>::proto_serialize(self.texture_height.as_ref().unwrap(), stream)?;
            <i32 as ProtoCodecVAR>::proto_serialize(
                self.x_tex_coordinate.as_ref().unwrap(),
                stream,
            )?;
            <i32 as ProtoCodecVAR>::proto_serialize(
                self.y_tex_coordinate.as_ref().unwrap(),
                stream,
            )?;
            {
                let vec = self.pixels.as_ref().unwrap();
                let len: u32 = vec.len().try_into()?;
                <u32 as ProtoCodecVAR>::proto_serialize(&len, stream)?;
                for i in vec {
                    <u32 as ProtoCodecVAR>::proto_serialize(i, stream)?;
                }
            }
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let map_id = ActorUniqueID::proto_deserialize(stream)?;
        let type_flags = <u32 as ProtoCodecVAR>::proto_deserialize(stream)?;
        let dimension = i8::proto_deserialize(stream)?;
        let is_locked = bool::proto_deserialize(stream)?;
        let map_origin = BlockPos::proto_deserialize(stream)?;

        let map_id_list = match type_flags & Type::Creation as u32 != 0 {
            true => {
                let len = <u32 as ProtoCodecVAR>::proto_deserialize(stream)?;
                let mut vec = Vec::with_capacity(len.try_into()?);
                for _ in 0..len {
                    vec.push(ActorUniqueID::proto_deserialize(stream)?);
                }

                Some(vec)
            }
            false => None,
        };

        let scale = match type_flags
            & (Type::DecorationUpdate as u32 | Type::TextureUpdate as u32 | Type::Creation as u32)
            != 0
        {
            true => Some(i8::proto_deserialize(stream)?),
            false => None,
        };

        let (actor_ids, decoration_list) = match type_flags & Type::DecorationUpdate as u32 != 0 {
            true => {
                let actor_ids = {
                    let len = <u32 as ProtoCodecVAR>::proto_deserialize(stream)?;
                    let mut vec = Vec::with_capacity(len.try_into()?);
                    for _ in 0..len {
                        vec.push(MapItemTrackedActorUniqueID::proto_deserialize(stream)?);
                    }
                    vec
                };
                let decoration_list = {
                    let len = <u32 as ProtoCodecVAR>::proto_deserialize(stream)?;
                    let mut vec = Vec::with_capacity(len.try_into()?);
                    for _ in 0..len {
                        vec.push(MapDecoration::proto_deserialize(stream)?);
                    }
                    vec
                };
                (Some(actor_ids), Some(decoration_list))
            }
            false => (None, None),
        };

        let (texture_width, texture_height, x_tex_coordinate, y_tex_coordinate, pixels) =
            match type_flags & Type::TextureUpdate as u32 != 0 {
                true => {
                    let texture_width = <i32 as ProtoCodecVAR>::proto_deserialize(stream)?;
                    let texture_height = <i32 as ProtoCodecVAR>::proto_deserialize(stream)?;
                    let x_tex_coordinate = <i32 as ProtoCodecVAR>::proto_deserialize(stream)?;
                    let y_tex_coordinate = <i32 as ProtoCodecVAR>::proto_deserialize(stream)?;
                    let pixels = {
                        let len = <u32 as ProtoCodecVAR>::proto_deserialize(stream)?;
                        let mut vec = Vec::with_capacity(len.try_into()?);
                        for _ in 0..len {
                            vec.push(<u32 as ProtoCodecVAR>::proto_deserialize(stream)?);
                        }
                        vec
                    };

                    (
                        Some(texture_width),
                        Some(texture_height),
                        Some(x_tex_coordinate),
                        Some(y_tex_coordinate),
                        Some(pixels),
                    )
                }
                false => (None, None, None, None, None),
            };

        Ok(Self {
            map_id,
            type_flags,
            dimension,
            is_locked,
            map_origin,
            map_id_list,
            scale,
            actor_ids,
            decoration_list,
            texture_width,
            texture_height,
            x_tex_coordinate,
            y_tex_coordinate,
            pixels,
        })
    }

    fn get_size_prediction(&self) -> usize {
        self.map_id.get_size_prediction()
            + <u32 as ProtoCodecVAR>::get_size_prediction(&self.type_flags)
            + self.dimension.get_size_prediction()
            + self.is_locked.get_size_prediction()
            + self.map_origin.get_size_prediction()
            + match &self.type_flags & Type::Creation as u32 != 0 {
            true => {
                size_of::<u32>()
                    + self
                    .map_id_list
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|i| i.get_size_prediction())
                    .sum::<usize>()
            }
            false => 0,
        }
            + match &self.type_flags
            & (Type::DecorationUpdate as u32
            | Type::TextureUpdate as u32
            | Type::Creation as u32)
            != 0
        {
            true => self.scale.as_ref().unwrap().get_size_prediction(),
            false => 0,
        }
            + match &self.type_flags & Type::DecorationUpdate as u32 != 0 {
            true => {
                size_of::<u32>()
                    + self
                    .actor_ids
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|i| i.get_size_prediction())
                    .sum::<usize>()
                    + size_of::<u32>()
                    + self
                    .decoration_list
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|i| i.get_size_prediction())
                    .sum::<usize>()
            }
            false => 0,
        }
            + match &self.type_flags & Type::TextureUpdate as u32 != 0 {
            true => {
                <i32 as ProtoCodecVAR>::get_size_prediction(&self.texture_width.unwrap())
                    + <i32 as ProtoCodecVAR>::get_size_prediction(&self.texture_height.unwrap())
                    + <i32 as ProtoCodecVAR>::get_size_prediction(
                    &self.x_tex_coordinate.unwrap(),
                )
                    + <i32 as ProtoCodecVAR>::get_size_prediction(
                    &self.y_tex_coordinate.unwrap(),
                )
                    + size_of::<u32>()
                    + &self.pixels.as_ref().unwrap().len() * size_of::<u32>()
            }
            false => 0,
        }
    }
}

// VERIFY: ProtoCodec impl
