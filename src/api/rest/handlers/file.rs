use crate::api::rest::prelude::*;

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
