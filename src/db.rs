// use response::Debug;
use rocket::fairing::{self, AdHoc};
use rocket::{Build, Rocket};
use rocket_sync_db_pools::Config;
use sqlx::{migrate, ConnectOptions};

type Db = sqlx::SqlitePool;
// type DbResult<T, E = Debug<Error>> = Result<T, E>;

async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
    let config = match Config::from("sqlx", &rocket) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to read SQLx config: {}", e);
            return Err(rocket);
        }
    };

    let mut opts = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(&config.url)
        .create_if_missing(true);

    opts.disable_statement_logging();
    let db = match Db::connect_with(opts).await {
        Ok(db) => db,
        Err(e) => {
            error!("Failed to connect to SQLx database: {}", e);
            return Err(rocket);
        }
    };

    if let Err(e) = migrate!("db/migrations").run(&db).await {
        error!("Failed to initialize SQLx database: {}", e);
        return Err(rocket);
    }

    Ok(rocket.manage(db))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(AdHoc::try_on_ignite("SQLx Database", init_db))
    })
}
