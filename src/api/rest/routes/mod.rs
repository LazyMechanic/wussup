mod auth;
mod health_check;
mod middleware;

use crate::api::rest::prelude::*;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    base_path()
        .and(
            health_check::health_check()
                .or(auth::login(ctx.clone()))
                .or(auth::logout(ctx.clone()))
                .or(auth::refresh_tokens(ctx)),
        )
        .boxed()
}

fn base_path() -> BoxedFilter<()> {
    warp::path("api").boxed()
}
