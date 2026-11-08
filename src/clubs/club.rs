use std::borrow::Cow;
use std::collections::{HashSet, VecDeque};
use heed::{BytesDecode, BytesEncode};
use serde::{Deserialize, Serialize};
use crate::accounting::account::Account;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Club {
    pub id: u64,
    pub created: u64,
    pub name: String,
    pub owner_id: HashSet<u64>,
    pub member_ids: HashSet<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubCreation {
    pub name: String,
    pub owner_id: HashSet<u64>,
}

impl ClubCreation {
    pub fn into_club_appllication(id: u64) -> ClubCreation {
        
    }
}

impl BytesEncode<'_> for Club {
    type EItem = Self;

    fn bytes_encode(item: &Self::EItem) -> Result<Cow<'_, [u8]>, Box<dyn std::error::Error + Send + Sync>> {
        let config = bincode::config::standard();
        let bytes = bincode::serde::encode_to_vec(item, config)?;
        Ok(Cow::Owned(bytes))
    }
}

impl<'a> BytesDecode<'a> for Club {
    type DItem = Self;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, Box<dyn std::error::Error + Send + Sync>> {
        // Modern bincode 2.0 implementation
        let config = bincode::config::standard();
        let (account, _) = bincode::serde::decode_from_slice(bytes, config)?;
        Ok(account)
    }
}
