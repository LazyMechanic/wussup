use super::middleware;
use crate::api::rest::prelude::*;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    get_settings(ctx.clone())
        .or(get_platforms(ctx.clone()))
        .or(get_builds(ctx.clone()))
        .or(add_platform(ctx.clone()))
        .or(add_build(ctx.clone()))
        .or(update_settings(ctx))
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

fn add_platform(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "settings" / "platforms")
        .and(warp::post())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_auth(ctx))
        .and(warp::body::json())
        .and_then(handlers::settings::add_platform)
        .boxed()
}

fn add_build(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "settings" / "builds")
        .and(warp::post())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_auth(ctx))
        .and(warp::body::json())
        .and_then(handlers::settings::add_build)
        .boxed()
}

fn update_settings(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "settings")
        .and(warp::put())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_auth(ctx))
        .and(warp::body::json())
        .and_then(handlers::settings::update_settings)
        .boxed()
}
