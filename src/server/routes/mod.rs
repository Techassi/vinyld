use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use nanoid::nanoid;

use crate::{
    store::Store,
    types::{CreateVinylRequest, CreateVinylResponse, Vinyl},
};

pub async fn create_vinyl(
    Extension(store): Extension<Store>,
    Json(payload): Json<CreateVinylRequest>,
) -> impl IntoResponse {
    let vinyl = Vinyl {
        id: nanoid!(),
        title: payload.title,
        artists: payload.artists,
        release_year: payload.release_year,
        tracks: payload.tracks,
        bought_in_condition: payload.bought_in_condition,
        media_condition: payload.media_condition,
        sleeve_condition: payload.sleeve_condition,
        digital_files_included: payload.digital_files_included,
    };

    return match store.create_vinyl(vinyl).await {
        Ok(_) => (
            StatusCode::OK,
            CreateVinylResponse::success("Vinyl created"),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            CreateVinylResponse::error("Internal Server Error"),
        ),
    };
}

pub async fn get_vinyls() -> impl IntoResponse {
    return StatusCode::OK;
}

pub async fn get_vinyl(Path(user_id): Path<String>) -> impl IntoResponse {
    return StatusCode::OK;
}

pub async fn update_vinyl() -> impl IntoResponse {
    return StatusCode::OK;
}

pub async fn delete_vinyl() -> impl IntoResponse {
    return StatusCode::OK;
}
