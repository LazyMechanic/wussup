mod auth;
mod file;
mod health_check;
mod middleware;
mod settings;

use crate::api::rest::prelude::*;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    base_path()
        .and(
            health_check::routes()
                .or(auth::routes(ctx.clone()))
                .or(settings::routes(ctx.clone()))
                .or(file::routes(ctx)),
        )
        .boxed()
}

fn base_path() -> BoxedFilter<()> {
    warp::path("api").boxed()
}
