pub mod handlers;
pub mod models;
pub mod routes;

mod prelude;

use std::net::{Ipv4Addr, SocketAddr};

use crate::config::Config;
use prelude::*;

pub async fn run(ctx: Context, cfg: Config) {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_method("GET")
        .allow_method("PUT")
        .allow_method("POST")
        .allow_method("DELETE")
        .allow_method("OPTIONS")
        .build();
    let log = warp::log("wussup::api");
    let routes = routes::routes(ctx)
        .with(log)
        .with(cors)
        .recover(models::Error::unpack);

    let addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), cfg.server.port);

    warp::serve(routes).run(addr).await;
}
