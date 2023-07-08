use axum::{
    middleware,
    routing::{get, post},
    Router, http::StatusCode,
};

use super::guard::guard_middleware;
use crate::generics::Pool;

#[derive(Clone)]
pub struct UserState {
    pub name: String,
}

pub fn mount_router(db_connection_pool: Pool) -> Router {
    Router::new()
        .layer(middleware::from_fn(guard_middleware))
        .with_state(db_connection_pool)
        .route("/", get(|| async {(StatusCode::OK, ("Root route :D"))}))
}

