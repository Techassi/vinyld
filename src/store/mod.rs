use sqlx::{pool::PoolConnection, postgres::PgPool, query, query_as, Pool, Postgres};

use crate::{
    config::StoreOptions,
    store::error::StoreError,
    types::{BuyCondition, Condition, Media, MediaType, RawMedia},
};

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

    async fn get_conn(&self) -> Result<PoolConnection<Postgres>, StoreError> {
        return match self.pool.acquire().await {
            Ok(conn) => Ok(conn),
            Err(err) => Err(StoreError::new(format!(
                "Failed to acquire database connection: {}",
                err
            ))),
        };
    }

    fn dsn(opts: StoreOptions) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            opts.username, opts.password, opts.host, opts.database
        )
    }

    pub async fn migrate(&self) -> Result<(), StoreError> {
        // Create the media table
        match query!(
            "CREATE TABLE IF NOT EXISTS media (
                id VARCHAR(22) PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                media_type VARCHAR(32) NOT NULL,
                catalogue VARCHAR(255) NOT NULL,
                release_date TIMESTAMP NOT NULL,
                purchase_date TIMESTAMP NOT NULL,
                media_condition VARCHAR(32) NOT NULL,
                sleeve_condition VARCHAR(32) NOT NULL,
                bought VARCHAR(32) NOT NULL,
                created_at TIMESTAMP NOT NULL,
                modified_at TIMESTAMP NOT NULL,
                notes TEXT NOT NULL
            );"
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => {}
            Err(err) => {
                return Err(StoreError::new(format!(
                    "Failed to create 'media' table: {}",
                    err
                )))
            }
        };

        // Create the media - artist relation table
        match query!(
            "CREATE TABLE IF NOT EXISTS media_artists_rel (
                id SERIAL PRIMARY KEY,
                media_id VARCHAR(22) NOT NULL,
                artist_id VARCHAR(22) NOT NULL
            );"
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => {}
            Err(err) => {
                return Err(StoreError::new(format!(
                    "Failed to create 'media_artists_rel' table: {}",
                    err
                )))
            }
        }

        // Create the artists table
        match query!(
            "CREATE TABLE IF NOT EXISTS artists (
                id VARCHAR(22) PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                urls TEXT
            );"
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => {}
            Err(err) => {
                return Err(StoreError::new(format!(
                    "Failed to create the 'artists' table: {}",
                    err
                )))
            }
        }

        // Create the media - tracks relation table
        match query!(
            "CREATE TABLE IF NOT EXISTS media_tracks_rel (
                id SERIAL PRIMARY KEY,
                media_id VARCHAR(22) NOT NULL,
                track_id VARCHAR(22) NOT NULL
            );"
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => {}
            Err(err) => {
                return Err(StoreError::new(format!(
                    "Failed to create 'media_tracks_rel' table: {}",
                    err
                )))
            }
        }

        // Create track table
        match query!(
            "CREATE TABLE IF NOT EXISTS tracks (
                id VARCHAR(22) PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                duration INT NOT NULL,
                record_side VARCHAR(2),
                digital BOOLEAN NOT NULL,
                urls TEXT
            );"
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => {}
            Err(err) => {
                return Err(StoreError::new(format!(
                    "Failed to create 'tracks' table: {}",
                    err
                )))
            }
        }

        // Create the media - label relation table
        match query!(
            "CREATE TABLE IF NOT EXISTS media_label_rel (
                id SERIAL PRIMARY KEY,
                media_id VARCHAR(22) NOT NULL,
                label_id VARCHAR(22) NOT NULL
            );"
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => {}
            Err(err) => {
                return Err(StoreError::new(format!(
                    "Failed to create 'media_label_rel' table: {}",
                    err
                )))
            }
        }

        // Create the labels table
        match query!(
            "CREATE TABLE IF NOT EXISTS labels (
                id VARCHAR(22) PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                label_code VARCHAR(255),
                urls TEXT
            );"
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => {}
            Err(_) => todo!(),
        }

        Ok(())
    }

    pub async fn create_media(&self, vinyl: Media) -> Result<(), StoreError> {
        Ok(())
    }

    pub async fn get_media_entries(&self) -> Result<Vec<Media>, StoreError> {
        let raw_media_entries = match query_as!(
            RawMedia,
            r#"
                SELECT
                    id, title, media_type as "media_type: MediaType",
                    catalogue, release_date, purchase_date,
                    media_condition as "media_condition: Condition",
                    sleeve_condition as "sleeve_condition: Condition",
                    bought as "bought: BuyCondition",
                    created_at, modified_at, notes
                FROM media
            "#
        )
        .fetch_all(&self.pool)
        .await
        {
            Ok(media_entries) => media_entries,
            Err(err) => {
                return Err(StoreError::new(format!(
                    "Failed to fetch media entries from 'media' table: {}",
                    err
                )))
            }
        };

        // let media_entry_ids = media_entries;

        let media_entries = raw_media_entries
            .iter()
            .map(|e| Media::from(e))
            .collect::<Vec<Media>>();

        Ok(media_entries)
    }

    pub async fn get_media_entry(&self, id: String) -> Result<Media, StoreError> {
        todo!()
    }

    pub async fn update_media_entry(&self, id: String, new_vinyl: Media) -> Result<(), StoreError> {
        Ok(())
    }

    pub async fn delete_media_entry(&self, id: String) -> Result<(), StoreError> {
        Ok(())
    }
}
