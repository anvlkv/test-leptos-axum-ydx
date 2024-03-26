use axum::{
    extract::{Path, State},
    response::Response,
};

use crate::AppState;

pub async fn create_entry(State(state): State<AppState>) -> Response {
    todo!()
}

pub async fn list_entries(State(state): State<AppState>) -> Response {
    todo!()
}

pub async fn get_entry(State(state): State<AppState>, Path(id): Path<i32>) -> Response {
    todo!()
}

pub async fn update_entry(State(state): State<AppState>, Path(id): Path<i32>) -> Response {
    todo!()
}
