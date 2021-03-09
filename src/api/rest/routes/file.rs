use super::middleware;
use crate::api::rest::prelude::*;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    get_files(ctx.clone()).boxed()
}

fn get_files(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("v1" / "files")
        .and(warp::get())
        .and(middleware::with_context(ctx.clone()))
        .and_then(handlers::file::get_files)
        .boxed()
}
