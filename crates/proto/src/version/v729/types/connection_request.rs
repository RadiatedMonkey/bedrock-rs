use std::collections::BTreeMap;
use std::io::{Cursor, Read};
use std::net::SocketAddr;
use std::string::FromUtf8Error;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bedrockrs_proto_core::error::{LoginError, ProtoCodecError};
use bedrockrs_proto_core::ProtoCodec;
use byteorder::{LittleEndian, ReadBytesExt};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use serde_json::Value;
use varint_rs::VarintReader;
use p384::pkcs8::spki;
use uuid::Uuid;
use bedrockrs_addon::language::code::LanguageCode;
use crate::v662::enums::{BuildPlatform, InputMode, UIProfile};
use crate::v662::types::{BaseGameVersion, SerializedSkin};

pub const MOJANG_PUBLIC_KEY: &str = "MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAECRXueJeTDqNRRgJi/vlRufByu/2G0i2Ebt6YMar5QX/R0DIIyrJMcUpruK4QveTfJSTp3Shlq4Gk34cD/4GUWwkv0DVuzeuB+tXija7HBxii03NHDbPAD0AKnLr2wdAp";

#[derive(Debug, Clone)]
pub struct ConnectionRequest {
    // /// Array of Base64 encoded JSON Web Token certificates to authenticate the player.
    // ///
    // /// The last certificate in the chain will have a property 'extraData' that contains player identity information including the XBL XUID (if the player was signed in to XBL at the time of the connection).
    // pub certificate_chain: Vec<BTreeMap<String, Value>>,
    // /// Base64 encoded JSON Web Token that contains other relevant client properties.
    // ///
    // /// Properties Include:
    // /// - SelfSignedId
    // /// - ServerAddress = (unresolved url if applicable)
    // /// - ClientRandomId
    // /// - SkinId
    // /// - SkinData
    // /// - SkinImageWidth
    // /// - SkinImageHeight
    // /// - CapeData
    // /// - CapeImageWidth
    // /// - CapeImageHeight
    // /// - SkinResourcePatch
    // /// - SkinGeometryData
    // /// - SkinGeometryDataEngineVersion
    // /// - SkinAnimationData
    // /// - PlayFabId
    // /// - AnimatedImageData = Array of:
    // ///   - Type
    // ///   - Image
    // ///   - ImageWidth
    // ///   - ImageHeight
    // ///   - Frames
    // ///   - AnimationExpression
    // /// - ArmSize
    // /// - SkinColor
    // /// - PersonaPieces = Array of:
    // ///   - PackId
    // ///   - PieceId
    // ///   - IsDefault
    // ///   - PieceType
    // ///   - ProductId
    // /// - PieceTintColors = Array of:
    // ///   - PieceType
    // ///   - Colors = Array of color hexstrings
    // /// - IsEduMode (if edu mode)
    // /// - TenantId (if edu mode)
    // /// - ADRole (if edu mode)
    // /// - IsEditorMode
    // /// - GameVersion
    // /// - DeviceModel
    // /// - DeviceOS = (see enumeration: BuildPlatform)
    // /// - DefaultInputMode = (see enumeration: InputMode)
    // /// - CurrentInputMode = (see enumeration: InputMode)
    // /// - UIProfile = (see enumeration: UIProfile)
    // /// - GuiScale
    // /// - LanguageCode
    // /// - PlatformUserId
    // /// - ThirdPartyName
    // /// - ThirdPartyNameOnly
    // /// - PlatformOnlineId
    // /// - PlatformOfflineId
    // /// - DeviceId
    // /// - TrustedSkin
    // /// - PremiumSkin
    // /// - PersonaSkin
    // /// - OverrideSkin
    // /// - CapeOnClassicSkin
    // /// - CapeId
    // /// - CompatibleWithClientSideChunkGen
    pub info: ClientInfo,
    pub xuid: String,
    pub uuid: Uuid,
    pub display_name: String,
    pub public_key: String
    // pub skin: Skin // TODO: Skin
}

#[derive(Deserialize, Debug)]
struct CertChain {
    pub chain: Vec<String>
}

#[derive(Deserialize, Debug)]
struct KeyPayload {
    #[serde(rename = "identityPublicKey")]
    pub public_key: String
}

fn parse_first_token(token: &str) -> Result<bool, ProtoCodecError> {
    let header = jsonwebtoken::decode_header(token)?;
    let Some(base64_x5u) = header.x5u else {
        return Err(LoginError::MissingX5U.into())
    };
    let bytes = BASE64_STANDARD.decode(base64_x5u)?;

    let public_key = spki::SubjectPublicKeyInfoRef::try_from(bytes.as_ref()).map_err(|e| {
        LoginError::InvalidPublicKey(e)
    })?;

    let decoding_key = DecodingKey::from_ec_der(public_key.subject_public_key.raw_bytes());
    let mut validation = Validation::new(Algorithm::ES384);
    validation.validate_exp = true;
    validation.validate_nbf = true;

    // Decode token
    let payload = jsonwebtoken::decode::<KeyPayload>(token, &decoding_key, &validation)?;
    Ok(payload.claims.public_key.eq(&MOJANG_PUBLIC_KEY))
}

fn parse_mojang_token(token: &str) -> Result<String, ProtoCodecError> {
    let bytes = BASE64_STANDARD.decode(MOJANG_PUBLIC_KEY)?;
    let public_key = spki::SubjectPublicKeyInfoRef::try_from(bytes.as_ref()).map_err(|e| {
        LoginError::InvalidPublicKey(e)
    })?;

    let decoding_key = DecodingKey::from_ec_der(public_key.subject_public_key.raw_bytes());
    let mut validation = Validation::new(Algorithm::ES384);
    validation.set_issuer(&["Mojang"]);
    validation.validate_nbf = true;
    validation.validate_exp = true;

    let payload = jsonwebtoken::decode::<KeyPayload>(token, &decoding_key, &validation)?;
    Ok(payload.claims.public_key)
}

#[derive(Deserialize, Debug)]
pub struct RawIdentity {
    #[serde(rename = "XUID")]
    pub xuid: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "identity")]
    pub uuid: Uuid
}

#[derive(Deserialize, Debug)]
struct IdentityPayload {
    #[serde(rename = "extraData")]
    pub client_data: RawIdentity,
    #[serde(rename = "identityPublicKey")]
    pub public_key: String
}

fn parse_identity_token(token: &str, key: &str) -> Result<IdentityPayload, ProtoCodecError> {
    let bytes = BASE64_STANDARD.decode(key)?;
    let public_key = spki::SubjectPublicKeyInfoRef::try_from(bytes.as_ref()).map_err(|e| {
        LoginError::InvalidPublicKey(e)
    })?;

    let decoding_key = DecodingKey::from_ec_der(public_key.subject_public_key.raw_bytes());
    let mut validation = Validation::new(Algorithm::ES384);
    validation.set_issuer(&["Mojang"]);
    validation.validate_exp = true;
    validation.validate_nbf = true;

    let payload = jsonwebtoken::decode::<IdentityPayload>(token, &decoding_key, &validation)?;
    Ok(payload.claims)
}

fn parse_identity(stream: &mut Cursor<&[u8]>) -> Result<IdentityPayload, ProtoCodecError> {
    let cert_chain_json_len = stream.read_i32::<LittleEndian>()?;
    let mut cert_chain_json = Vec::with_capacity(cert_chain_json_len as usize);
    cert_chain_json.resize(cert_chain_json_len as usize, 0);

    stream.read_exact(&mut cert_chain_json)?;

    let cert_chain = serde_json::from_slice::<CertChain>(&cert_chain_json)?;

    let identity;
    match cert_chain.chain.len() {
        // User is offline
        1 => {
            return Err(LoginError::UserOffline.into())
        },
        // Authenticated with Microsoft services
        3 => {
            // Verify the first token and use its public key for the next token.

            let valid = parse_first_token(&cert_chain.chain[0])?;
            if !valid {
                // Login attempted using forged token chain.
                return Err(LoginError::NotSignedByMojang.into())
            }

            let key = parse_mojang_token(&cert_chain.chain[1])?;
            identity = parse_identity_token(&cert_chain.chain[2], &key)?;
        },
        // This should not happen...
        len => {
            return Err(LoginError::InvalidChainLength(len).into())
        }
    }

    Ok(identity)
}

#[derive(Deserialize, Debug, Clone)]
pub struct ClientInfo {
    #[serde(rename = "ClientRandomId")]
    pub client_random_id: i64,
    #[serde(rename = "CompatibleWithClientSideChunkGen")]
    pub client_side_chunk_gen_compatible: bool,
    #[serde(rename = "CurrentInputMode")]
    pub current_input_mode: InputMode,
    #[serde(rename = "DefaultInputMode")]
    pub default_input_mode: InputMode,
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    #[serde(rename = "DeviceModel")]
    pub device_model: String,
    #[serde(rename = "DeviceOS")]
    pub build_platform: BuildPlatform,
    #[serde(rename = "GameVersion")]
    pub game_version: String,
    #[serde(rename = "GuiScale")]
    pub gui_scale: i32,
    #[serde(rename = "IsEditorMode")]
    pub editor_mode: bool,
    #[serde(rename = "LanguageCode", with = "language_code")]
    pub language_code: LanguageCode,
    #[serde(rename = "MaxViewDistance")]
    pub max_view_distance: u32,
    #[serde(rename = "MemoryTier")]
    pub memory_tier: u8,
    #[serde(rename = "PlatformOfflineId")]
    pub platform_offline_id: String,
    #[serde(rename = "PlatformOnlineId")]
    pub platform_online_id: String,
    #[serde(rename = "PlatformType")]
    pub platform_type: u8,
    #[serde(rename = "SelfSignedId")]
    pub self_signed_id: Uuid,
    #[serde(rename = "ServerAddress")]
    pub server_address: SocketAddr,
    #[serde(rename = "ThirdPartyName")]
    pub third_party_name: String,
    #[serde(rename = "ThirdPartyNameOnly")]
    pub third_party_name_only: bool,
    #[serde(rename = "UIProfile")]
    pub ui_profile: UIProfile,
}

mod language_code {
    use bedrockrs_addon::language::code::LanguageCode;
    use serde::{Deserialize, Deserializer};
    
    #[inline]
    pub fn deserialize<'de, D>(de: D) -> Result<LanguageCode, D::Error> 
        where D: Deserializer<'de>
    {
        let lang = String::deserialize(de)?;
        Ok(LanguageCode::VanillaCode(lang))
    }
}

fn parse_client_info_token(token: &str, key: &str) -> Result<ClientInfo, ProtoCodecError> {
    let bytes = BASE64_STANDARD.decode(key)?;
    let public_key = spki::SubjectPublicKeyInfoRef::try_from(bytes.as_ref()).map_err(|e| {
        LoginError::InvalidPublicKey(e)
    })?;

    let decoding_key = DecodingKey::from_ec_der(public_key.subject_public_key.raw_bytes());
    let mut validation = Validation::new(Algorithm::ES384);

    validation.required_spec_claims.clear();

    dbg!(token);
    let payload = jsonwebtoken::decode(token, &decoding_key, &validation)?;
    Ok(payload.claims)
}

fn parse_client_info(stream: &mut Cursor<&[u8]>, public_key: &str) -> Result<ClientInfo, ProtoCodecError> {
    let token_len = stream.read_i32::<LittleEndian>()?;
    let mut token = Vec::with_capacity(token_len as usize);
    token.resize(token_len as usize, 0);

    stream.read_exact(&mut token)?;

    let token_str = std::str::from_utf8(&token)?;
    parse_client_info_token(token_str, public_key)
}

impl ProtoCodec for ConnectionRequest {
    fn proto_serialize(&self, _stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        todo!()
    }

    // TODO: Add microsoft auth
    // TODO: Validate jwts (This is hard, Zuri nor Vincent could help me)
    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        stream.read_u32_varint()?;

        let identity = parse_identity(stream)?;
        let user_data = parse_client_info(stream, &identity.public_key)?;

        let login = Self {
            display_name: identity.client_data.display_name,
            xuid: identity.client_data.xuid,
            uuid: identity.client_data.uuid,
            info: user_data,
            public_key: identity.public_key
        };

        dbg!(&login);

        Ok(login)
    }

    fn get_size_prediction(&self) -> usize {
        // TODO
        1
    }
}
