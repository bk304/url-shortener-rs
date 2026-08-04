use crate::{
    AppState,
    handlers::{create_url, delete_url, get_info_url, get_original_url},
};
use axum::{Router, routing::get, routing::post};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/urls", post(create_url))
        .route("/urls/{token}", get(get_info_url).delete(delete_url))
        .route("/{token}", get(get_original_url))
        .with_state(app_state)
}
