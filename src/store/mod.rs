use sqlx::{pool::PoolConnection, postgres::PgPool, Pool, Postgres};

use crate::{config::StoreOptions, store::error::StoreError, types::Vinyl};

mod error;

#[derive(Clone)]
pub struct Store {
    pool: Pool<Postgres>,
}

impl Store {
    pub async fn new(opts: StoreOptions) -> Result<Self, StoreError> {
        let url = Self::dsn(opts);
        let pool = match PgPool::connect(url.as_str()).await {
            Ok(pool) => pool,
            Err(err) => {
                return Err(StoreError::new(format!(
                    "Failed to connect to database: {}",
                    err
                )))
            }
        };

        let store = Self { pool };
        Ok(store)
    }

    pub fn dsn(opts: StoreOptions) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            opts.username, opts.password, opts.host, opts.database
        )
    }

    async fn get_conn(&self) -> Result<PoolConnection<Postgres>, StoreError> {
        return match self.pool.acquire().await {
            Ok(conn) => Ok(conn),
            Err(err) => Err(StoreError::new(format!(
                "Failed to acquire database connection: {}",
                err
            ))),
        };
    }

    pub async fn create_vinyl(&self, vinyl: Vinyl) -> Result<(), StoreError> {
        Ok(())
    }

    pub async fn get_vinyls(&self) -> Result<Vec<Vinyl>, StoreError> {
        let vinyls: Vec<Vinyl> = Vec::new();
        Ok(vinyls)
    }

    pub async fn get_vinyl(&self, id: String) -> Result<Vinyl, StoreError> {
        let vinyl = Vinyl {
            id,
            title: todo!(),
            artists: todo!(),
            release_year: todo!(),
            tracks: todo!(),
            // bought_at: todo!(),
            bought_in_condition: todo!(),
            media_condition: todo!(),
            sleeve_condition: todo!(),
            digital_files_included: todo!(),
        };
        Ok(vinyl)
    }

    pub async fn update_vinyl(&self, id: String, new_vinyl: Vinyl) -> Result<(), StoreError> {
        Ok(())
    }

    pub async fn delete_vinyl(&self, id: String) -> Result<(), StoreError> {
        Ok(())
    }
}
