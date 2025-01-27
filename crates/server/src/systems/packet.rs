use bedrockrs_proto::connection::shard::arc::ConnectionShared;
use bedrockrs_proto::ProtoHelper;
use shipyard::{Component, ViewMut};

#[derive(Component)]
pub struct Connected<T: ProtoHelper + 'static>
where
    <T as ProtoHelper>::GamePacketType: Sync,
{
    pub connection: ConnectionShared<T>,
}

pub fn packet_recv_system<T: ProtoHelper>(v_con: ViewMut<Connected<T>>)
where
    <T as ProtoHelper>::GamePacketType: Sync,
{
}

pub fn packet_send_system<T: ProtoHelper>(v_con: ViewMut<Connected<T>>)
where
    <T as ProtoHelper>::GamePacketType: Sync,
{
}
