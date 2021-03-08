use crate::api::rest::prelude::*;
use crate::services::prelude::*;

pub async fn get_settings(ctx: Context) -> responses::Json {
    let settings = ctx
        .settings_service
        .get_settings()
        .await
        .map_err(models::Error::err_with_internal_error)?;

    let settings: models::settings::Settings = settings.into();

    Ok(responses::settings::GetSettings::new(settings).into_json())
}

pub async fn get_platforms(ctx: Context) -> responses::Json {
    let platforms = ctx
        .settings_service
        .get_platforms()
        .await
        .map_err(models::Error::err_with_internal_error)?;

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
        .map_err(models::Error::err_with_internal_error)?;

    let builds = builds.into_iter().map(|b| b.name).collect::<Vec<String>>();

    Ok(responses::settings::GetBuilds::new(builds).into_json())
}
