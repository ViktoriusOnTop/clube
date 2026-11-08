use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::{Extension, Router};
use axum::routing::post;
use axum_governor::{nz, GovernorConfigBuilder, PeerIp, Quota, GovernorLayer};
use opaque_ke::argon2::password_hash::rand_core::OsRng;
use opaque_ke::{ServerLogin, ServerSetup};
use tokio::net::TcpListener;
use tokio::sync::OnceCell;
use crate::accounting::account::{DefaultCipherSuite};
use crate::accounting::database::account_db::AccountsDB;
use crate::accounting::database::emailid_db::EmailIdDB;
use crate::accounting::database::usernameid_db::UsernameIdDB;
use crate::accounting::login::api::finish::login_finish;
use crate::accounting::login::api::start::login_start;
use crate::accounting::signup::api::finish::signup_finish;
use crate::accounting::signup::api::start::signup_start;

static ACCOUNTS_DB: OnceCell<AccountsDB> = OnceCell::const_new();
static EMAILID_DB: OnceCell<EmailIdDB> = OnceCell::const_new();
static USERNAMEID_DB: OnceCell<UsernameIdDB> = OnceCell::const_new();

pub mod accounting;
pub mod clubs;

struct AppState {
    accounts_db: Arc<AccountsDB>,
    email_db: Arc<EmailIdDB>,
    username_db: Arc<UsernameIdDB>,
    setup: Arc<ServerSetup<DefaultCipherSuite>>,
    ongoing_logins: Mutex<HashMap<String, ServerLogin<DefaultCipherSuite>>>,
}

#[tokio::main]
async fn main() {
    let app = Router::new();

    ACCOUNTS_DB.get_or_init(|| async {
        AccountsDB::new().expect("failed to initialize accounts database")
    }).await;
    EMAILID_DB.get_or_init(|| async {
        EmailIdDB::new().expect("failed to initialize email database")
    }).await;
    USERNAMEID_DB.get_or_init(|| async {
        UsernameIdDB::new().expect("failed to initialize username database")
    }).await;

    let acc_db_ref = ACCOUNTS_DB.get().expect("database was never initialized... thats wierd time, for another 10 hour session of banging my head against the desk to fix this :P");
    let emailid_db_ref = EMAILID_DB.get().expect("database was never initialized... thats wierd time, for another 10 hour session of banging my head against the desk to fix this :P");
    let userid_db_ref = USERNAMEID_DB.get().expect("database was never initialized... thats wierd, time for another 10 hour session of banging my head against the desk to fix this :P");

    let server_setup = get_server_setup();

    let state = AppState{
        accounts_db: Arc::new(AccountsDB{
            env: acc_db_ref.env.clone(),
            db: acc_db_ref.db,
        }),
        email_db: Arc::new(EmailIdDB{
            env: emailid_db_ref.env.clone(),
            db: emailid_db_ref.db,
        }),
        username_db: Arc::new(UsernameIdDB{
            env: userid_db_ref.env.clone(),
            db: userid_db_ref.db,
        }),
        setup: Arc::new(server_setup),
        ongoing_logins: Mutex::new(HashMap::new()),
    };

    let signup_governor = GovernorConfigBuilder::default()
        .with_extractor(PeerIp::default())
        .expect_connect_info()
        .quota_default(Quota::requests_per_second(nz!(2u32)))
        .finish()
        .unwrap();
    let app = Router::new()
        .route("/api/signup/start", post(signup_start))
        .route("/api/signup/finish", post(signup_finish))
        .layer(GovernorLayer::new(signup_governor))
        .route("/auth/login/start", post(login_start))
        .route("/auth/login/finish", post(login_finish))
        .layer(Extension(Arc::new(state)));

    let listener = match TcpListener::bind("0.0.0.0:12212").await {
        Ok(l) => l,
        Err(_) => {
            println!("smth is stealing my port :<");
            return;
        }
    };

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
        .await
        .expect("app start failed, is the port taken? no then the tcplistener wouldn't bind nvm idk :P");
}

fn get_server_setup() -> ServerSetup<DefaultCipherSuite> {
    let home_dir = home::home_dir().expect("could not find home directory =[");
    let key_dir = home_dir.join(".clubs/accounting/server_setup/key");
    if key_dir.exists() {
        std::fs::read(key_dir).expect("failed to read server_setup key").into()
    }else {
        let mut rng = OsRng;
        ServerSetup::<DefaultCipherSuite>::new(&mut rng)
    }
}