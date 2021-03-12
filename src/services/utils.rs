use chrono::{Duration, NaiveDateTime, Utc};
use std::path::PathBuf;

pub fn expires_timestamp(sec_duration: i64) -> NaiveDateTime {
    (Utc::now() + Duration::seconds(sec_duration)).naive_utc()
}

pub fn format_file_name<S1, S2, S3>(platform: S1, build: S2, version: S3) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    format!(
        "{}-{}-{}",
        platform.as_ref(),
        build.as_ref(),
        version.as_ref()
    )
}
