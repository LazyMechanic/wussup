use crate::api::rest::prelude::*;

pub fn health_check() -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("health-check")
        .and(warp::get())
        .and_then(handlers::health_check::health_check)
        .boxed()
}
