use std::collections::{HashSet, VecDeque};

pub struct Club {
    pub id: u64,
    pub created: u64,
    pub name: String,
    pub owner_id: HashSet<u64>,
    pub member_ids: HashSet<u64>,
}