use anyhow::Context;
use chrono::{DateTime, NaiveDateTime, Utc};

pub fn parse_db_datetime(value: &str) -> anyhow::Result<DateTime<Utc>> {
    if let Ok(datetime) = DateTime::parse_from_rfc3339(value) {
        return Ok(datetime.with_timezone(&Utc));
    }

    if let Ok(datetime) = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S") {
        return Ok(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc));
    }

    Err(anyhow::anyhow!("Unsupported time format: {}", value))
        .context("Failed to parse database datetime")
}
