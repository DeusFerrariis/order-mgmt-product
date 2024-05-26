use std::{env, fmt::Error};

use sqlx::{pool::PoolOptions, sqlite::SqlitePoolOptions, Sqlite};

pub type DB = sqlx::Pool<Sqlite>;

#[derive(Debug, Clone)]
pub struct DatabaseProvider {
    pub db: DB,
}

impl DatabaseProvider {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub async fn new_sqlite(path: String) -> Result<Self, ()> {
        let rwc_path = format!("{}?mode=rwc", path);
        let pool = sqlx::sqlite::SqlitePool::connect(rwc_path.as_str()).await;
        match pool {
            Ok(p) => Ok(Self { db: p }),
            Err(e) => {
                println!("{:?}", e);
                Err(())
            }
        }
    }
}
