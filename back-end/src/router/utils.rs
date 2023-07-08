use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BacksonResponse<T: Serialize> {
    message: String,
    code: u16,
    data: Option<T>,
}

pub fn response_message(
    status_code: StatusCode,
    message: String,
    additional_data: Option<impl Serialize>,
) -> impl IntoResponse {
    let response = BacksonResponse {
        data: additional_data,
        code: status_code.as_u16(),
        message: message,
    };

    (status_code, Json(response).into_response())
}

pub fn internal_error<E: std::error::Error>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
