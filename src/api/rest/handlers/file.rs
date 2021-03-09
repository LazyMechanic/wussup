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
