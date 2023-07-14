use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    database::models::user::{InsertableUser, UserQueries},
    generics::Pool,
    router::utils::{internal_error, response_message},
};

pub async fn create_user(
    State(pool): State<Pool>,
    Json(user): Json<InsertableUser>,
) -> impl IntoResponse {
    let connection = pool.get_owned().await.map_err(internal_error).unwrap();
    let mut user_queries = UserQueries {
        connection: connection,
    };

    let user_exists = user_queries.get_user_by_email(user.email.clone()).await;

    if user_exists.is_ok() {
        return response_message(
            StatusCode::BAD_REQUEST,
            "User already exists with this email...".to_owned(),
            None,
        );
    }

    let inserted_user_result = user_queries.inser_user(user).await;

    match inserted_user_result {
        Ok(user) => response_message(
            StatusCode::OK,
            "Your user was created successfully!".to_owned(),
            Some(user),
        ),
        Err(e) => response_message(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "Something went wrong while inserting user {}",
                e.to_string()
            ),
            None,
        ),
    }
}
