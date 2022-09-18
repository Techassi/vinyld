use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use nanoid::nanoid;

use crate::{
    constants,
    store::Store,
    types::{GetMediaEntriesResponse, GetMediaEntryResponse},
};

pub async fn create_media_entry(
    Extension(store): Extension<Store>,
    // Json(payload): Json<CreateVinylRequest>,
) -> impl IntoResponse {
    // let vinyl = Media {
    //     id: nanoid!(),
    //     title: payload.title,
    //     artists: payload.artists,
    //     release_year: payload.release_year,
    //     tracks: payload.tracks,
    //     bought_in_condition: payload.bought_in_condition,
    //     media_condition: payload.media_condition,
    //     sleeve_condition: payload.sleeve_condition,
    //     digital_files_included: payload.digital_files_included,
    // };

    // return match store.create_vinyl(vinyl).await {
    //     Ok(_) => (
    //         StatusCode::OK,
    //         CreateVinylResponse::success("Vinyl created"),
    //     ),
    //     Err(_) => (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         CreateVinylResponse::error("Internal Server Error"),
    //     ),
    // };
}

pub async fn get_media_entries(Extension(store): Extension<Store>) -> impl IntoResponse {
    let media_entries = match store.get_media_entries(0, 0).await {
        Ok(media_entries) => media_entries,
        Err(_) => todo!(),
    };

    return (
        StatusCode::OK,
        GetMediaEntriesResponse::success(media_entries),
    );
}

pub async fn get_media_entry(
    Path(media_id): Path<String>,
    Extension(store): Extension<Store>,
) -> impl IntoResponse {
    if media_id.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            GetMediaEntryResponse::error("Missing media ID"),
        );
    }

    if media_id.len() != constants::NANOID_LEN {
        return (
            StatusCode::BAD_REQUEST,
            GetMediaEntryResponse::error("Invalid media ID"),
        );
    }

    match store.get_media_entry(media_id).await {
        Ok(media_entry) => return (StatusCode::OK, GetMediaEntryResponse::success(media_entry)),
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                GetMediaEntryResponse::error(err.to_string()),
            );
        }
    };
}

pub async fn update_media_entry() -> impl IntoResponse {
    return StatusCode::OK;
}

pub async fn delete_media_entry() -> impl IntoResponse {
    return StatusCode::OK;
}
