pub mod auth;
pub mod settings;

use crate::api::rest::prelude::*;

// Use responses::Custom<impl warp::Reply> for universal response
pub type Custom<T> = Result<T, warp::reject::Rejection>;
pub type Empty = Custom<models::Nothing>;
pub type Json = Custom<warp::reply::Json>;

pub trait EmptyExt {
    fn ok() -> Empty;
    fn err(e: warp::reject::Rejection) -> Empty;
}

impl EmptyExt for Empty {
    fn ok() -> Empty {
        Ok(models::Nothing::new())
    }

    fn err(e: warp::reject::Rejection) -> Empty {
        Err(e)
    }
}

pub trait IntoWarpJsonResponse {
    fn into_json(self) -> warp::reply::Json;
}

impl<T> IntoWarpJsonResponse for T
where
    T: serde::Serialize,
{
    fn into_json(self) -> warp::reply::Json {
        warp::reply::json(&self)
    }
}
