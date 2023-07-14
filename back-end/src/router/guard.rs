use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{database::models::auth_token::AuthTokenQueries, generics::Pool};

use super::{router::UserState, utils::internal_error};

pub async fn guard_middleware<T>(
    mut request: Request<T>,
    next: Next<T>,
    State(pool): State<Pool>,
) -> Result<Response, (StatusCode, String)> {
    let connection = pool.get_owned().await.map_err(internal_error).unwrap();

    let header_token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or((
            StatusCode::BAD_REQUEST,
            "Authorization header malformed".to_owned(),
        ))?
        .token()
        .to_owned();

    let token_user = AuthTokenQueries { connection }
        .get_auth_user_by_token(header_token)
        .await;

    match token_user {
        Ok(logged_user) => {
            request
                .extensions_mut()
                .insert(UserState { user: logged_user });
            Ok(next.run(request).await)
        }
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            "Token does't match with any users".to_owned(),
        )),
    }
}
