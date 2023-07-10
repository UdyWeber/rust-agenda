use std::env::var;

use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::generics::Pool;

use super::{router::UserState, utils::internal_error};

pub async fn guard_middleware<T>(
    mut request: Request<T>,
    next: Next<T>,
    // State(pool): State<Pool>,
) -> Result<Response, (StatusCode, String)> {
    // let connection = pool
    //     .get_owned()
    //     .await
    //     .map_err(internal_error)
    //     .unwrap();

    let auth_token = match var("REQUEST_AUTH_TOKEN") {
        Ok(token) => token,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    // Setting Up Middle Extension to get in other routes
    request.extensions_mut().insert(UserState {
        name: "Waj".to_owned(),
    });

    // Token Based Authorization using typed_headers
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or((
            StatusCode::BAD_REQUEST,
            "Authorization header malformed".to_owned(),
        ))?
        .token()
        .to_owned();

    // Matching auth
    match auth_token == token {
        true => Ok(next.run(request).await),
        false => Err((
            StatusCode::UNAUTHORIZED,
            "Token doesnt match with requirements".to_owned(),
        )),
    }
}
