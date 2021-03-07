use super::middleware;
use crate::api::rest::prelude::*;

pub fn login(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "auth" / "login")
        .and(warp::post())
        .and(middleware::with_context(ctx))
        .and(warp::body::json())
        .and_then(handlers::auth::login)
        .boxed()
}

pub fn logout(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "auth" / "logout")
        .and(warp::post())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_jwt(ctx))
        .and_then(handlers::auth::logout)
        .boxed()
}

pub fn refresh_tokens(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "auth" / "refresh-tokens")
        .and(warp::post())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_jwt(ctx))
        .and(warp::body::json())
        .and_then(handlers::auth::refresh_tokens)
        .boxed()
}
