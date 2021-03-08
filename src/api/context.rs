use crate::services::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub auth_service: Arc<AuthService>,
    pub settings_service: Arc<SettingsService>,
}
