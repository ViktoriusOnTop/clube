use std::collections::HashSet;
use opaque_ke::{ciphersuite::CipherSuite, generic_array::GenericArray, ServerRegistrationLen, ServerRegistration};
use opaque_ke::argon2::Argon2;

pub struct DefaultCipherSuite;

impl CipherSuite for DefaultCipherSuite {
    type OprfCs = opaque_ke::Ristretto255;
    type KeyExchange = opaque_ke::TripleDh<opaque_ke::Ristretto255, sha2::Sha512>;
    type Ksf = Argon2<'static>;
}

pub struct Account {
    pub id: u64,
    pub created_at: u64, //generic info
    pub updated_at: u64,
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
    pub failed_login_attempts_time: u64,

    pub joined_clubs_ids: HashSet<u64>, //club ids :D
    pub owned_clubs_ids: HashSet<u64>,
}
