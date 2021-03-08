use super::middleware;
use crate::api::rest::prelude::*;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    get_settings(ctx.clone())
        .or(get_platforms(ctx.clone()))
        .or(get_builds(ctx))
        .boxed()
}

fn get_settings(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "settings")
        .and(warp::get())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_auth(ctx))
        .and_then(handlers::settings::get_settings)
        .boxed()
}

fn get_platforms(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "settings" / "platforms")
        .and(warp::get())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_auth(ctx))
        .and_then(handlers::settings::get_platforms)
        .boxed()
}

fn get_builds(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "settings" / "builds")
        .and(warp::get())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_auth(ctx))
        .and_then(handlers::settings::get_builds)
        .boxed()
}
