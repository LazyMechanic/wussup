use crate::api::rest::prelude::*;

// Use responses::Custom<impl warp::Reply> for universal response
pub type Custom<T> = Result<T, warp::reject::Rejection>;
pub type Empty = Custom<models::Nothing>;
pub type Json = Custom<warp::reply::Json>;

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
