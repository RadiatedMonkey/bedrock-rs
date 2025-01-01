mod handler;

use crate::error::LoginError;
use crate::login::handler::LoginHandler;
use bedrockrs_proto::connection::shard::arc::shard;
use bedrockrs_proto::connection::Connection;
use bedrockrs_proto::v729::helper::ProtoHelperV729;
use shipyard::World;

pub async fn login(
    connection: Connection,
    world: &mut World,
    login_handler: impl LoginHandler,
) -> Result<(), LoginError> {
    let mut shard = shard::<ProtoHelperV729>(connection);

    todo!()
}
