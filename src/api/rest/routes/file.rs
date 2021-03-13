use super::middleware;
use crate::api::rest::prelude::*;
use uuid::Uuid;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    get_files(ctx.clone())
        .or(upload(ctx.clone()))
        .or(download(ctx.clone()))
        .or(delete(ctx))
        .boxed()
}

fn get_files(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "files")
        .and(warp::get())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_auth(ctx))
        .and_then(handlers::file::get_files)
        .boxed()
}

fn upload(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "files" / String / String / String)
        .and(warp::post())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_auth(ctx))
        .and(warp::body::stream())
        .and_then(handlers::file::upload)
        .boxed()
}

fn download(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "files" / String / String / String)
        .and(warp::get())
        .and(middleware::with_context(ctx))
        .and_then(handlers::file::download)
        .boxed()
}

fn delete(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "files" / String / String / String)
        .and(warp::delete())
        .and(middleware::with_context(ctx.clone()))
        .and(middleware::with_auth(ctx))
        .and_then(handlers::file::delete)
        .boxed()
}
