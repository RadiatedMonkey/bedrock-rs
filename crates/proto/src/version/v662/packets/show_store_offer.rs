use crate::version::v662::enums::ShowStoreOfferRedirectType;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 91)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct ShowStoreOfferPacket {
    pub product_id: String,
    pub redirect_type: ShowStoreOfferRedirectType,
}