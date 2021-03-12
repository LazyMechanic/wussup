use crate::api::rest::prelude::*;

pub async fn get_settings(ctx: Context) -> api_models::JsonResponse {
    let settings = ctx
        .settings_service
        .get_settings()
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let resp: api_models::settings::GetSettingsResponse = settings.into();

    Ok(resp.into_json())
}

pub async fn get_platforms(ctx: Context) -> api_models::JsonResponse {
    let platforms = ctx
        .settings_service
        .get_platforms()
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let resp: api_models::settings::GetPlatformsResponse = platforms.into();

    Ok(resp.into_json())
}

pub async fn get_builds(ctx: Context) -> api_models::JsonResponse {
    let builds = ctx
        .settings_service
        .get_builds()
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let resp: api_models::settings::GetBuildsResponse = builds.into();

    Ok(resp.into_json())
}

pub async fn create_platform(
    ctx: Context,
    req: api_models::settings::CreatePlatformRequest,
) -> api_models::JsonResponse {
    let platforms = ctx
        .settings_service
        .create_platform(req.into())
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let resp: api_models::settings::CreatePlatformResponse = platforms.into();

    Ok(resp.into_json())
}

pub async fn create_build(
    ctx: Context,
    req: api_models::settings::CreateBuildRequest,
) -> api_models::JsonResponse {
    let builds = ctx
        .settings_service
        .create_build(req.into())
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let resp: api_models::settings::CreateBuildResponse = builds.into();

    Ok(resp.into_json())
}

pub async fn delete_platform(name: String, ctx: Context) -> api_models::JsonResponse {
    let platforms = ctx
        .settings_service
        .delete_platform(&name)
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let resp: api_models::settings::DeletePlatformResponse = platforms.into();

    Ok(resp.into_json())
}

pub async fn delete_build(name: String, ctx: Context) -> api_models::JsonResponse {
    let builds = ctx
        .settings_service
        .delete_build(&name)
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let resp: api_models::settings::DeleteBuildResponse = builds.into();

    Ok(resp.into_json())
}

pub async fn update_settings(
    ctx: Context,
    req: api_models::settings::UpdateSettingsRequest,
) -> api_models::JsonResponse {
    let req_settings = req.into();

    let settings = ctx
        .settings_service
        .update_settings(req_settings)
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let resp: api_models::settings::UpdateSettingsResponse = settings.into();

    Ok(resp.into_json())
}
