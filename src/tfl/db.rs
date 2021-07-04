use rocket::{Build, Rocket, fairing::{self, AdHoc}};
use rocket_sync_db_pools::Config;
use sqlx::{migrate};

type Db = sqlx::PgPool;

async fn init_tfl_db(rocket: Rocket<Build>) -> fairing::Result {
    let config = match Config::from("tfl", &rocket) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to read SQLx TFL config: {}", e);
            return Err(rocket);
        }
    };

    println!("Im config, {:?}", config);

    let url = config.url;

    let db = match Db::connect(&url).await {
        Ok(db) => db,
        Err(e) => {
            error!("Failed to connect to TFL SQLx database: {}", e);
            return Err(rocket);
        }
    };

    if let Err(e) = migrate!("db/tfl/migrations").run(&db).await {
        error!("Failed to initialize TFL SQLx database: {}", e);
        return Err(rocket);
    }

    Ok(rocket.manage(db))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(AdHoc::try_on_ignite("SQLx TFL Database", init_tfl_db))
    })
}