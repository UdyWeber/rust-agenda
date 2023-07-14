use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use super::{
    guard::guard_middleware,
    routes::{login::login, user::create_user, wellcome_route::wellcome},
};
use crate::{database::models::user::SelectableUser, generics::Pool};

#[derive(Clone)]
pub struct UserState {
    pub user: SelectableUser,
}

pub fn mount_router(db_connection_pool: Pool) -> Router {
    Router::new()
        .route("/", get(wellcome))
        .layer(middleware::from_fn_with_state(
            db_connection_pool.clone(),
            |state, req, next| guard_middleware(req, next, state),
        ))
        .route("/user", post(create_user))
        .route("/login", post(login))
        .with_state(db_connection_pool)
}
