use heed::byteorder::BigEndian;
use heed::types::{Str, U64};
use heed::{CommitOrAbort, Database, Env, EnvOpenOptions, PutFlags};
use std::fs;
use std::path::PathBuf;

pub struct UsernameIdDB {
    pub(crate) env: Env,
    pub(crate) db: Database<'static, Str, U64<BigEndian>>,
}

fn get_db_dir() -> PathBuf {
    let home_dir = home::home_dir().expect("could not find home directory =[");
    home_dir.join(".clubs/accounting/database/usernameiddb")
}

fn normalise(username: &str) -> String {
    username.trim().to_lowercase()
}

impl UsernameIdDB {
    pub fn new() -> anyhow::Result<Self> {
        let path = get_db_dir();
        fs::create_dir_all(&path)?;

        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(1024 * 1024 * 1024)
                .max_dbs(2)
                .open(path)?
        };

        let wtxn = env.write_txn()?;
        let db = env.create_databases_and_commit(wtxn, |wdo, wtxn| {
            wdo.database_options()
                .types::<Str, U64<BigEndian>>()
                .name("username-to-id")
                .create(wtxn)
                .map(CommitOrAbort::<_, ()>::Commit)
        })?
            .unwrap_commit();
        Ok(Self { env, db })
    }

    pub fn insert_usernameid(&self, username: &str, id: u64) -> heed::Result<()> {
        let mut wtxn = self.env.write_txn()?;
        self.db
            .put_with_flags(&mut wtxn, PutFlags::NO_OVERWRITE, &normalise(username), &id)?;
        wtxn.commit()
    }

    pub fn find_by_username(&self, username: &str) -> heed::Result<Option<u64>> {
        let rtxn = self.env.read_txn()?;
        self.db.get(&rtxn, &normalise(username))
    }
}
