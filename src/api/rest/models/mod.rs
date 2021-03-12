pub mod auth;
pub mod file;
pub mod settings;

use http_api_problem::ApiError;
use http_api_problem::HttpApiProblem;
use http_api_problem::PROBLEM_JSON_MEDIA_TYPE;

// Use responses::Custom<impl warp::Reply> for universal response
pub type CustomResponse<T> = Result<T, warp::reject::Rejection>;
pub type EmptyResponse = CustomResponse<Nothing>;
pub type JsonResponse = CustomResponse<warp::reply::Json>;

pub trait EmptyExt {
    fn ok() -> EmptyResponse;
    fn err(e: warp::reject::Rejection) -> EmptyResponse;
}

impl EmptyExt for EmptyResponse {
    fn ok() -> EmptyResponse {
        Ok(Nothing::new())
    }

    fn err(e: warp::reject::Rejection) -> EmptyResponse {
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

#[derive(serde::Serialize)]
pub struct Nothing {}

impl Nothing {
    pub fn new() -> Nothing {
        Nothing {}
    }
}

impl warp::Reply for Nothing {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::Response::new("".into())
    }
}

pub struct Error;

impl Error {
    #[allow(dead_code)]
    pub fn err_with_internal_error<E>(err: E) -> warp::reject::Rejection
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Error::err_with_status(http::StatusCode::INTERNAL_SERVER_ERROR, err)
    }

    #[allow(dead_code)]
    pub fn err_with_status<S, E>(status: S, err: E) -> warp::reject::Rejection
    where
        S: Into<http::StatusCode>,
        E: std::error::Error + Send + Sync + 'static,
    {
        log::error!("internal error occurred: {:?}", err);
        warp::reject::custom(ApiError::with_cause(status, err))
    }

    #[allow(dead_code)]
    pub fn msg_with_internal_error<M>(msg: M) -> warp::reject::Rejection
    where
        M: Into<String>,
    {
        Error::msg_with_status(http::StatusCode::INTERNAL_SERVER_ERROR, msg)
    }

    #[allow(dead_code)]
    pub fn msg_with_status<S, M>(status: S, msg: M) -> warp::reject::Rejection
    where
        S: Into<http::StatusCode>,
        M: Into<String>,
    {
        let msg = msg.into();
        log::error!("internal error occurred: {:?}", msg);
        warp::reject::custom(ApiError::with_message(status, msg))
    }

    pub async fn unpack(
        rejection: warp::reject::Rejection,
    ) -> Result<impl warp::Reply, std::convert::Infallible> {
        let reply = if rejection.is_not_found() {
            let problem =
                HttpApiProblem::with_title_and_type_from_status(http::StatusCode::NOT_FOUND);
            reply_from_problem(&problem)
        } else if let Some(err) = rejection.find::<ApiError>() {
            let problem = err.to_http_api_problem();
            reply_from_problem(&problem)
        } else if let Some(e) = rejection.find::<warp::filters::body::BodyDeserializeError>() {
            let problem = HttpApiProblem::new("Invalid Request Body")
                .set_status(http::StatusCode::BAD_REQUEST)
                .set_detail(format!("Request body is invalid: {}", e));
            reply_from_problem(&problem)
        } else if rejection.find::<warp::reject::MethodNotAllowed>().is_some() {
            let problem = HttpApiProblem::with_title_and_type_from_status(
                http::StatusCode::METHOD_NOT_ALLOWED,
            );
            reply_from_problem(&problem)
        } else {
            let problem = HttpApiProblem::with_title_and_type_from_status(
                http::StatusCode::INTERNAL_SERVER_ERROR,
            );
            reply_from_problem(&problem)
        };

        Ok(reply)
    }
}

fn reply_from_problem(problem: &HttpApiProblem) -> impl warp::Reply {
    let code = problem
        .status
        .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR);

    let reply = warp::reply::json(problem);
    let reply = warp::reply::with_status(reply, code);
    warp::reply::with_header(reply, http::header::CONTENT_TYPE, PROBLEM_JSON_MEDIA_TYPE)
}
