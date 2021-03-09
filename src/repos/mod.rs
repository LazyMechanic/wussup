pub mod auth;
pub mod error;
pub mod prelude;
pub mod settings;

pub use crate::repos::error::RepoError;

use crate::config;
use crate::repos::auth::AuthRepo;
use crate::repos::file::FileRepo;
use crate::repos::settings::SettingsRepo;

pub type Db = sqlx::Postgres;
pub type DbPool = sqlx::Pool<Db>;
pub type DbPoolConnection = sqlx::pool::PoolConnection<Db>;

pub async fn connect(cfg: &config::Db) -> Result<DbPool, RepoError> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(cfg.max_connections)
        .connect(&cfg.url)
        .await?;

    Ok(pool)
}

macro_rules! ext_impl{
    ($ext_type:ident, $(($repo_type:ty, $f:ident),)*) => {
        pub trait $ext_type {
            $(
            fn $f(&self) -> $repo_type;
            )*
        }

        impl $ext_type for DbPool {
            $(
            fn $f(&self) -> $repo_type {
                <$repo_type>::new(self)
            }
            )*
        }
    }
}

ext_impl!(
    DbPoolExt,
    (SettingsRepo, settings_repo),
    (AuthRepo, auth_repo),
);
