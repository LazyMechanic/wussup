use crate::api::rest::prelude::*;
use uuid::Uuid;

pub async fn get_files(ctx: Context) -> api_models::JsonResponse {
    let files = ctx
        .file_service
        .get_files()
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let files: Vec<api_models::file::File> = files.into_iter().map(|f| f.into()).collect();

    Ok(api_models::file::GetFilesResponse { files }.into_json())
}

pub async fn upload<S, B>(
    platform: String,
    build: String,
    version: String,
    ctx: Context,
    stream: S,
) -> api_models::JsonResponse
where
    S: warp::Stream<Item = Result<B, warp::Error>> + Unpin,
    B: warp::Buf,
{
    let files = ctx
        .file_service
        .upload(stream, platform, build, version)
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let resp: api_models::file::UploadFileResponse = files.into();

    Ok(resp.into_json())
}

pub async fn download(
    platform: String,
    build: String,
    version: String,
    ctx: Context,
) -> api_models::CustomResponse<impl warp::Reply> {
    let file = ctx
        .file_service
        .download(platform, build, version)
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let body = hyper::Body::wrap_stream(file);
    let resp = warp::reply::Response::new(body);

    Ok(resp)
}

pub async fn delete(
    platform: String,
    build: String,
    version: String,
    ctx: Context,
) -> api_models::JsonResponse {
    let files = ctx
        .file_service
        .delete(platform, build, version)
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let resp: api_models::file::DeleteFileResponse = files.into();

    Ok(resp.into_json())
}
