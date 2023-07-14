use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    database::models::{
        auth_token::InsertableAuthToken,
        user::{LoginData, UserQueries},
    },
    generics::Pool,
    router::utils::{internal_error, response_message},
    utils::security::match_passwords,
};

// State always comes first
pub async fn login(
    State(pool): State<Pool>,
    Json(login_data): Json<LoginData>,
) -> impl IntoResponse {
    let connection = pool.get_owned().await.map_err(internal_error).unwrap();

    let mut user_queries = UserQueries {
        connection: connection,
    };

    let user_result = user_queries.get_user_by_email(login_data.email).await;

    if user_result.is_err() {
        return response_message(
            StatusCode::OK,
            "User with this email does not exists in the database".to_owned(),
            None,
        );
    }

    let user = user_result.unwrap();

    match match_passwords(login_data.password, user.password_salt, user.password) {
        true => {
            let auth_token = InsertableAuthToken::new(user.id);

            user_queries.insert_auth_token(auth_token.clone()).await;

            response_message(
                StatusCode::OK,
                "Credentials are Right, Authenticated!".to_owned(),
                Some(auth_token),
            )
        }
        false => response_message(
            StatusCode::UNAUTHORIZED,
            "Passwords do not match!".to_owned(),
            None,
        ),
    }
}
