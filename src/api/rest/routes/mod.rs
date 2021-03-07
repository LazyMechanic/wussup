mod health_check;

use crate::api::rest::prelude::*;

pub fn routes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
    base_path().and(health_check::health_check()).boxed()
}

fn base_path() -> BoxedFilter<()> {
    warp::path("api").boxed()
}
