use std::collections::HashMap;

use sqlx::{migrate, postgres::PgPool, query_as, Pool, Postgres};

use crate::{
    config::StoreOptions,
    store::{
        error::StoreError,
        models::{ArtistsJoin, MediaJoin, TracksJoin},
    },
    types::{Artist, BuyCondition, Condition, Media, MediaType, Track},
};

mod error;
pub mod models;

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

    fn dsn(opts: StoreOptions) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            opts.username, opts.password, opts.host, opts.database
        )
    }

    pub async fn migrate(&self) -> Result<(), StoreError> {
        return match migrate!().run(&self.pool).await {
            Ok(_) => Ok(()),
            Err(err) => Err(StoreError::new(format!("Migration failed: {}", err))),
        };
    }

    fn group_artists_by_media_id(artists: Vec<ArtistsJoin>) -> HashMap<String, Vec<Artist>> {
        let mut map: HashMap<String, Vec<Artist>> = HashMap::new();
        for artist in artists {
            if map.contains_key(&artist.media_id) {
                if let Some(v) = map.get_mut(&artist.media_id) {
                    v.push(Artist {
                        id: artist.artist_id,
                        name: artist.artist_name,
                        urls: artist.artist_urls,
                    })
                }
                continue;
            }

            let v: Vec<Artist> = vec![Artist {
                id: artist.artist_id,
                name: artist.artist_name,
                urls: artist.artist_urls,
            }];
            map.insert(artist.media_id, v);
        }

        return map;
    }

    fn group_tracks_by_media_id(tracks: Vec<TracksJoin>) -> HashMap<String, Vec<Track>> {
        let mut map: HashMap<String, Vec<Track>> = HashMap::new();
        for track in tracks {
            if map.contains_key(&track.media_id) {
                if let Some(v) = map.get_mut(&track.media_id) {
                    v.push(Track {
                        id: track.track_id,
                        title: track.track_title,
                        duration: track.track_duration,
                        record_side: track.track_record_side,
                        digital: track.track_digital,
                        urls: track.track_urls,
                    });
                }
                continue;
            }

            let v: Vec<Track> = vec![Track {
                id: track.track_id,
                title: track.track_title,
                duration: track.track_duration,
                record_side: track.track_record_side,
                digital: track.track_digital,
                urls: track.track_urls,
            }];
            map.insert(track.media_id, v);
        }

        return map;
    }

    pub async fn create_media(&self, media: Media) -> Result<(), StoreError> {
        Ok(())
    }

    pub async fn get_media_entries(
        &self,
        _offset: usize,
        _limit: usize,
    ) -> Result<Vec<Media>, StoreError> {
        let raw_media_entries = match query_as!(
            MediaJoin,
            r#"
                SELECT
                    media.id as media_id, media.title as media_title, media.media_type as "media_media_type: MediaType",
                    media.catalogue as media_catalogue, media.release_date as media_release_date,
                    media.purchase_date as media_purchase_date, media.media_condition as "media_media_condition: Condition",
                    media.sleeve_condition as "media_sleeve_condition: Condition", media.bought as "media_bought: BuyCondition",
                    media.created_at as media_created_at, media.modified_at as media_modified_at, media.notes as media_notes,
                    -- Label
                    labels.id as label_id, labels.name as label_name, labels.label_code as label_label_code,
                    labels.urls as label_urls
                    FROM media
                -- Label Joins
                JOIN media_label_rel ON (media_label_rel.media_id = media.id)
                JOIN labels ON (media_label_rel.label_id = labels.id)
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

        // Collect media ids
        let media_ids = raw_media_entries
            .iter()
            .map(|e| e.media_id.clone())
            .collect::<Vec<String>>();

        // Get artists data
        let artists = match query_as!(
            ArtistsJoin,
            r#"
                SELECT
                    artists.id as artist_id, artists.name as artist_name, artists.urls as artist_urls,
                    media_artists_rel.media_id as media_id
                FROM artists
                JOIN media_artists_rel ON (media_artists_rel.media_id = ANY($1))
                WHERE artists.id = media_artists_rel.artist_id
            "#,
            &media_ids[..]
        )
        .fetch_all(&self.pool)
        .await
        {
            Ok(artists) => artists,
            Err(_) => todo!(),
        };

        let artists = Self::group_artists_by_media_id(artists);
        println!("{}", artists.len());

        // Get tracks data
        let tracks = match query_as!(
            TracksJoin,
            r#"
                SELECT
                    tracks.id as track_id, tracks.title as track_title, tracks.duration as track_duration,
                    tracks.record_side as track_record_side, tracks.digital as track_digital,
                    tracks.urls as track_urls, tracks.belongs_to as media_id
                FROM tracks
                WHERE tracks.belongs_to = ANY($1)
            "#,
            &media_ids[..]
        )
        .fetch_all(&self.pool)
        .await
        {
            Ok(tracks) => tracks,
            Err(_) => todo!(),
        };

        let tracks = Self::group_tracks_by_media_id(tracks);

        let mut media_entries: Vec<Media> = Vec::new();
        for media_entry in raw_media_entries {
            let mut entry = Media::from(media_entry);

            if let Some(v) = artists.get(&entry.id) {
                entry.artists = v.to_vec();
            }

            if let Some(v) = tracks.get(&entry.id) {
                entry.tracks = v.to_vec();
            }

            media_entries.push(entry);
        }

        Ok(media_entries)
    }

    pub async fn get_media_entry(&self, id: String) -> Result<Media, StoreError> {
        // Get base data
        let raw_media_entry = match query_as!(
            MediaJoin,
            r#"
                SELECT
                    media.id as media_id, media.title as media_title, media.media_type as "media_media_type: MediaType",
                    media.catalogue as media_catalogue, media.release_date as media_release_date,
                    media.purchase_date as media_purchase_date, media.media_condition as "media_media_condition: Condition",
                    media.sleeve_condition as "media_sleeve_condition: Condition", media.bought as "media_bought: BuyCondition",
                    media.created_at as media_created_at, media.modified_at as media_modified_at, media.notes as media_notes,
                    -- Label
                    labels.id as label_id, labels.name as label_name, labels.label_code as label_label_code,
                    labels.urls as label_urls
                FROM media
                -- Label Joins
                JOIN media_label_rel ON (media_label_rel.media_id = media.id)
                JOIN labels ON (media_label_rel.label_id = labels.id)
                WHERE media.id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(media_entry) => media_entry,
            Err(err) => {
                return Err(StoreError::new(format!(
                    "Failed to fetch media entries from 'media' table: {}",
                    err
                )))
            }
        };

        // Get artists data
        let artists = match query_as!(
            ArtistsJoin,
            r#"
                SELECT
                    artists.id as artist_id, artists.name as artist_name, artists.urls as artist_urls,
                    media_artists_rel.media_id as media_id
                FROM artists
                JOIN media_artists_rel ON (media_artists_rel.media_id = $1)
                WHERE artists.id = media_artists_rel.artist_id
            "#,
            raw_media_entry.media_id
        )
        .fetch_all(&self.pool)
        .await
        {
            Ok(artists) => artists,
            Err(_) => todo!(),
        };

        let artists = Self::group_artists_by_media_id(artists);

        // Get tracks data
        let tracks = match query_as!(
            TracksJoin,
            r#"
                SELECT
                    tracks.id as track_id, tracks.title as track_title, tracks.duration as track_duration,
                    tracks.record_side as track_record_side, tracks.digital as track_digital,
                    tracks.urls as track_urls, tracks.belongs_to as media_id
                FROM tracks
                WHERE tracks.belongs_to = $1
            "#,
            raw_media_entry.media_id
        )
        .fetch_all(&self.pool)
        .await
        {
            Ok(tracks) => tracks,
            Err(_) => todo!(),
        };

        let tracks = Self::group_tracks_by_media_id(tracks);

        let mut media_entry = Media::from(raw_media_entry);
        if let Some(v) = artists.get(&media_entry.id) {
            media_entry.artists = v.to_vec();
        }
        if let Some(v) = tracks.get(&media_entry.id) {
            media_entry.tracks = v.to_vec();
        }

        Ok(media_entry)
    }

    pub async fn update_media_entry(&self, id: String, new_vinyl: Media) -> Result<(), StoreError> {
        Ok(())
    }

    pub async fn delete_media_entry(&self, id: String) -> Result<(), StoreError> {
        Ok(())
    }
}
