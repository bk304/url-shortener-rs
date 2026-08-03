use std::sync::Arc;
use axum::{Router, routing::post, routing::get};
use crate::{AppState, handlers::{create_url, get_original_url, delete_url, get_info_url}};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/urls", post(create_url))
        .route("/urls/{token}", get(get_info_url).delete(delete_url))
        .route("/{token}", get(get_original_url))
        .with_state(app_state)
}