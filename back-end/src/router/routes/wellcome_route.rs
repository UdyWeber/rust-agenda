use axum::{http::StatusCode, response::IntoResponse, Extension};

use crate::router::{router::UserState, utils::response_message};

pub async fn wellcome(Extension(state): Extension<UserState>) -> impl IntoResponse {
    response_message(
        StatusCode::OK,
        "Wellcome my fellow user! Look's like you have been authenticatied, you can check your data bellow :D!".to_owned(),
        Some(state.user),
    )
}
