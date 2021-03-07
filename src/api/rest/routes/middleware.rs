use crate::api::rest::prelude::*;
use crate::services::prelude::*;

use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub fn with_context(
    ctx: Context,
) -> impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || ctx.clone())
}

pub fn with_jwt(
    ctx: Context,
) -> impl Filter<Extract = (Jwt,), Error = warp::reject::Rejection> + Clone {
    warp::any()
        .map(move || Arc::clone(&ctx.auth_service))
        .and(warp::cookie::optional(REFRESH_TOKEN_COOKIE_NAME))
        .and(warp::header::optional::<String>("Authorization"))
        .and_then(
            |auth_service: Arc<AuthService>,
             cookie: Option<String>,
             header: Option<String>| async move {
                let cookie = cookie.ok_or_else(|| models::Error::msg_with_status(
                    http::StatusCode::UNAUTHORIZED,
                    format!("cookie not found, name={}", REFRESH_TOKEN_COOKIE_NAME),
                ))?;

                let header = header.ok_or_else(|| models::Error::msg_with_status(
                    http::StatusCode::UNAUTHORIZED,
                    "header Authorization not found",
                ))?;

                let claims = auth_service.authorize(&header).await.map_err(|err| {
                    models::Error::err_with_status(http::StatusCode::UNAUTHORIZED, err)
                })?;

                let refresh_token = Uuid::from_str(
                    &cookie::Cookie::parse(cookie)
                        .map_err(models::Error::err_with_internal_error)?
                        .value()
                        .to_string(),
                )
                    .map_err(models::Error::err_with_internal_error)?;

                Result::<_, warp::reject::Rejection>::Ok(Jwt {
                    claims,
                    refresh_token,
                })
            },
        )
}
