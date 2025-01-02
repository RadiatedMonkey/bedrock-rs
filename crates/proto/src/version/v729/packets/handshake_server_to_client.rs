use std::collections::BTreeMap;
use std::io::{Cursor, Write};

use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use serde_json::Value;
use varint_rs::VarintWriter;
// Yeah we aren't supporting secure things rn...

#[gamepacket(id = 3)]
#[derive(Debug, Clone)]
pub struct HandshakeServerToClientPacket {
    pub jwt: String
}

impl ProtoCodec for HandshakeServerToClientPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        stream.write_u32_varint(self.jwt.len() as u32)?;
        stream.write_all(self.jwt.as_bytes())?;

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        todo!()
    }

    fn get_size_prediction(&self) -> usize {
        300
    }
}

//
// impl MCProtoSerialize for HandshakeServerToClientPacket {
//     fn proto_serialize(&self) -> Result<Vec<u8>, SerilizationError>
//     where
//         Self: Sized,
//     {
//         let key = match EcdsaPrivateKey::generate(EcdsaAlgorithm::ES384) {
//             Ok(v) => v,
//             Err(_) => {
//                 return Err(SerilizationError::GenerateKeyError);
//             }
//         };
//
//         let mut jwt = jwtk::HeaderAndClaims::new_dynamic();
//
//         jwt.header_mut();
//
//         for (string, val) in self.handshake_webtoken.iter() {
//             jwt.insert(string, val.clone());
//         }
//
//         let token = match sign(&mut jwt, &key) {
//             Ok(v) => v,
//             Err(_) => {
//                 return Err(SerilizationError::JwtError);
//             }
//         };
//
//         println!("token:\n{}", token);
//
//         let mut buf = vec![];
//
//         match buf.write_u64_varint(token.len() as u64) {
//             Ok(_) => {}
//             Err(_) => {
//                 return Err(SerilizationError::WriteVarintError);
//             }
//         }
//
//         buf.put_slice(token.as_bytes());
//
//         Ok(buf)
//     }
// }
//
