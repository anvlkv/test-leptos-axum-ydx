use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, patch, post};
use axum::Router;

use crate::handlers::entries::{create_entry, get_entry, list_entries, update_entry};
use crate::AppState;

pub fn api_router() -> Router<AppState> {
    Router::new().nest("/entries", entries_routes())
}

fn entries_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_entry))
        .route("/", get(list_entries))
        .route("/:id", get(get_entry))
        .route("/:id", patch(update_entry))
}
