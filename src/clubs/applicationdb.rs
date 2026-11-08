use crate::accounting::account::{Account, AccountCreation};
use heed::byteorder::BigEndian;
use heed::types::U64;
use heed::{CommitOrAbort, Database, Env, EnvOpenOptions, PutFlags};
use std::fs;
use std::path::PathBuf;
use crate::clubs::club::{Club, ClubCreation};

pub struct ClubsDB {
    pub(crate) env: Env,
    pub(crate) db: Database<'static, U64<BigEndian>, Club>
}

fn get_db_dir() -> PathBuf {
    let home_dir = home::home_dir().expect("could not find home directory =[");
    home_dir.join(".clubs/clubs/database/clubsdb")
}

impl ClubsDB {
    pub fn new() -> anyhow::Result<Self> {
        let path = get_db_dir();
        fs::create_dir_all(&path)?;

        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(10 * 1024 * 1024 * 1024)
                .max_dbs(2)
                .open(path)?
        };

        let wtxn = env.write_txn()?;
        let db = env.create_databases_and_commit(wtxn, |wdo, wtxn| {
            wdo
                .database_options()
                .types::<U64<BigEndian>, Club>()
                .name("accounts")
                .create(wtxn)
                .map(CommitOrAbort::<_, ()>::Commit)
        })?
            .unwrap_commit();

        Ok(Self { env, db })
    }

    pub fn create_account(&self, club: ClubCreation) -> heed::Result<u64> {
        let mut wtxn = self.env.write_txn()?;

        let id = self.db.last(&wtxn)?.map_or(0, |(last_id, _)| last_id + 1);

        let account = club.into_club_appllication(id);
        self.db.put(&mut wtxn, &id, &account)?;

        wtxn.commit()?;
        Ok(id)
    }

    pub fn find_account(&self, id: u64) -> heed::Result<Option<Club>> {
        let rtxn = self.env.read_txn()?;
        self.db.get(&rtxn, &id)
    }
}
