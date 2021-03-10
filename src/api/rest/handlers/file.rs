use crate::api::rest::prelude::*;

pub async fn get_files(ctx: Context) -> responses::Json {
    let files = ctx
        .file_service
        .get_files()
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let files: Vec<api_models::file::File> = files.into_iter().map(|f| f.into()).collect();

    Ok(responses::file::GetFiles::new(files).into_json())
}

pub async fn upload<S, B>(path: String, ctx: Context, stream: S) -> responses::Json
where
    S: warp::Stream<Item = Result<B, warp::Error>> + Unpin,
    B: warp::Buf,
{
    let files = ctx
        .file_service
        .upload(stream, path)
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let files = files
        .into_iter()
        .map(|f| api_models::file::File {
            id: f.id,
            path: f.path,
        })
        .collect();

    Ok(responses::file::UploadFile::new(files).into_json())
}
