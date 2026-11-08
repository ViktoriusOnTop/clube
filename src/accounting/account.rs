use heed::{BytesDecode, BytesEncode};
use opaque_ke::argon2::Argon2;
use opaque_ke::{ServerRegistration, ciphersuite::CipherSuite};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashSet;
use tokio::time::Instant;

pub struct DefaultCipherSuite;

impl CipherSuite for DefaultCipherSuite {
    type OprfCs = opaque_ke::Ristretto255;
    type KeyExchange = opaque_ke::TripleDh<opaque_ke::Ristretto255, sha2::Sha512>;
    type Ksf = Argon2<'static>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: u64,
    pub created_at: u64, //generic info
    pub updated_at: u64, //either updated or created at time
    pub last_login_at: Option<u64>,

    pub display_name: String, //name
    pub username: String,

    pub(super) server_registration: ServerRegistration<DefaultCipherSuite>, //password

    pub email: String, //email
    pub is_email_verified: bool,

    pub phone_number: Option<String>, //phone
    pub is_phone_verified: Option<bool>,

    pub last_active: Option<u64>, //last active, duh

    pub recent_failed_login_attempts: i32, //uh oh some idiots trying to hack into account :P
    pub failed_login_attempts_lockout_time: u64,

    pub joined_clubs_ids: HashSet<u64>, //club ids :D
    pub owned_clubs_ids: HashSet<u64>,
}

pub struct UsernameID{
    pub id: u64,
    pub username: String,
}

pub struct EmailID{
    pub id: u64,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCreation {
    pub last_login_at: Option<u64>,

    pub display_name: String, //name
    pub username: String,

    pub(super) server_registration: ServerRegistration<DefaultCipherSuite>, //password

    pub email: String, //email
    pub is_email_verified: bool,

    pub phone_number: Option<String>, //phone
    pub is_phone_verified: Option<bool>,

    pub last_active: Option<u64>, //last active, duh

    pub recent_failed_login_attempts: i32, //uh oh some idiots trying to hack into account :P
    pub failed_login_attempts_lockout_time: u64,

    pub joined_clubs_ids: HashSet<u64>, //club ids :D
    pub owned_clubs_ids: HashSet<u64>,
}

impl BytesEncode<'_> for Account {
    type EItem = Self;

    fn bytes_encode(item: &Self::EItem) -> Result<Cow<'_, [u8]>, Box<dyn std::error::Error + Send + Sync>> {
        let config = bincode::config::standard();
        let bytes = bincode::serde::encode_to_vec(item, config)?;
        Ok(Cow::Owned(bytes))
    }
}

impl<'a> BytesDecode<'a> for Account {
    type DItem = Self;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, Box<dyn std::error::Error + Send + Sync>> {
        // Modern bincode 2.0 implementation
        let config = bincode::config::standard();
        let (account, _) = bincode::serde::decode_from_slice(bytes, config)?;
        Ok(account)
    }
}

impl AccountCreation {
    pub fn into_account(self, id: u64) -> Account{
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let account = Account {
            id,
            created_at: now,
            updated_at: now,
            last_login_at: self.last_login_at,
            display_name: self.display_name,
            username: self.username,
            server_registration: self.server_registration,
            email: self.email,
            is_email_verified: self.is_email_verified,
            phone_number: self.phone_number,
            is_phone_verified: self.is_phone_verified,
            last_active: self.last_active,
            recent_failed_login_attempts: self.recent_failed_login_attempts,
            failed_login_attempts_lockout_time: self.failed_login_attempts_lockout_time,
            joined_clubs_ids: self.joined_clubs_ids,
            owned_clubs_ids: self.owned_clubs_ids,
        };
        account
    }
}