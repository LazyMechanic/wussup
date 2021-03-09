use crate::api::rest::prelude::*;

pub async fn get_settings(ctx: Context) -> responses::Json {
    let settings = ctx
        .settings_service
        .get_settings()
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let settings: api_models::settings::Settings = settings.into();

    Ok(responses::settings::GetSettings::new(settings).into_json())
}

pub async fn get_platforms(ctx: Context) -> responses::Json {
    let platforms = ctx
        .settings_service
        .get_platforms()
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let platforms = platforms
        .into_iter()
        .map(|p| p.name)
        .collect::<Vec<String>>();

    Ok(responses::settings::GetPlatforms::new(platforms).into_json())
}

pub async fn get_builds(ctx: Context) -> responses::Json {
    let builds = ctx
        .settings_service
        .get_builds()
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let builds = builds.into_iter().map(|b| b.name).collect::<Vec<String>>();

    Ok(responses::settings::GetBuilds::new(builds).into_json())
}

pub async fn add_platform(ctx: Context, req: requests::settings::AddPlatform) -> responses::Json {
    let r = db_models::settings::Platform { name: req.name };

    let platforms = ctx
        .settings_service
        .add_platform(r)
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let res = platforms.into_iter().map(|p| p.name).collect();

    Ok(responses::settings::AddPlatform::new(res).into_json())
}

pub async fn add_build(ctx: Context, req: requests::settings::AddBuild) -> responses::Json {
    let r = db_models::settings::Build { name: req.name };

    let builds = ctx
        .settings_service
        .add_build(r)
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let res = builds.into_iter().map(|p| p.name).collect();

    Ok(responses::settings::AddBuild::new(res).into_json())
}

pub async fn update_settings(
    ctx: Context,
    req: requests::settings::UpdateSettings,
) -> responses::Json {
    let req_settings = req.payload.into();

    let res_settings = ctx
        .settings_service
        .update_settings(req_settings)
        .await
        .map_err(api_models::Error::err_with_internal_error)?;

    let res_settings = res_settings.into();

    Ok(responses::settings::UpdateSettings::new(res_settings).into_json())
}
