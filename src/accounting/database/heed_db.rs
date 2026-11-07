use std::path::PathBuf;
use heed::{Database, Env};
use crate::accounting::account::Account;

pub struct HeedDb {
    env: Env,
    db: Database<u64, Account>
}

fn get_db_dir() -> PathBuf {
    let home_dir = home::home_dir().expect("could not find home directory =[");
    home_dir.join(".clubs/accounting/database")
}