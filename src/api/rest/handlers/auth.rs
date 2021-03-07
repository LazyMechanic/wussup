use chrono::Utc;
use time::Duration;

use crate::api::rest::prelude::*;
use crate::services::prelude::*;

pub async fn login(
    ctx: Context,
    req: requests::auth::Login,
) -> responses::Custom<impl warp::Reply> {
    log::debug!("login, req={:?}", req);

    let (access_token, refresh_token) = ctx
        .auth_service
        .login(req.fingerprint, req.password)
        .await
        .map_err(|err| models::Error::err_with_status(http::StatusCode::UNAUTHORIZED, err))?;

    let reply = warp::reply::json(&responses::auth::Login { access_token });
    let reply = reply_with_cookie(reply, refresh_token);

    Ok(reply)
}

pub async fn refresh_tokens(
    ctx: Context,
    jwt: Jwt,
    req: requests::auth::RefreshTokens,
) -> responses::Custom<impl warp::Reply> {
    log::debug!("refresh tokens, jwt={:?}, req={:?}", jwt, req);

    let (access_token, refresh_token) = ctx
        .auth_service
        .refresh_tokens(req.fingerprint, jwt)
        .await
        .map_err(|err| models::Error::err_with_status(http::StatusCode::UNAUTHORIZED, err))?;

    let reply = warp::reply::json(&responses::auth::RefreshTokens { access_token });
    let reply = reply_with_cookie(reply, refresh_token);

    Ok(reply)
}

fn reply_with_cookie(
    reply: impl warp::Reply,
    refresh_token: RefreshTokenEntry,
) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        http::header::SET_COOKIE,
        cookie::Cookie::build(REFRESH_TOKEN_COOKIE_NAME, refresh_token.token.to_string())
            .http_only(true)
            .max_age(Duration::seconds(
                refresh_token.exp - Utc::now().timestamp(),
            ))
            .finish()
            .to_string(),
    )
}

pub async fn logout(ctx: Context, jwt: Jwt) -> responses::Empty {
    log::debug!("logout, jwt={:?}", jwt);

    ctx.auth_service
        .logout(jwt)
        .await
        .map_err(models::Error::err_with_internal_error)?;

    Ok(models::Nothing::new())
}
