use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::repositories::user_settings_repository::UserSettingsRepository;
use domain::types::UserSettings;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct SQLiteUserSettingsRepository {
    pool: SqlitePool,
}

impl SQLiteUserSettingsRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserSettingsRepository for SQLiteUserSettingsRepository {
    async fn get(&self) -> anyhow::Result<UserSettings> {
        let row = sqlx::query_as::<_, UserSettingsRow>(
            r#"
                SELECT
                    monday_target_minutes,
                    tuesday_target_minutes,
                    wednesday_target_minutes,
                    thursday_target_minutes,
                    friday_target_minutes,
                    saturday_target_minutes,
                    sunday_target_minutes,
                    show_archived_projects,
                    show_archived_tasks,
                    created_at,
                    updated_at
                FROM user_settings
                WHERE id = 1
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        row.parse()
    }

    async fn update(&self, settings: UserSettings) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE user_settings
            SET 
                monday_target_minutes = ?,
                tuesday_target_minutes = ?,
                wednesday_target_minutes = ?,
                thursday_target_minutes = ?,
                friday_target_minutes = ?,
                saturday_target_minutes = ?,
                sunday_target_minutes = ?,
                show_archived_projects = ?,
                show_archived_tasks = ?,
                updated_at = ?
            WHERE id = 1
"#,
        )
        .bind(settings.monday_target_minutes)
        .bind(settings.tuesday_target_minutes)
        .bind(settings.wednesday_target_minutes)
        .bind(settings.thursday_target_minutes)
        .bind(settings.friday_target_minutes)
        .bind(settings.saturday_target_minutes)
        .bind(settings.sunday_target_minutes)
        .bind(if settings.show_archived_projects {
            1
        } else {
            0
        })
        .bind(if settings.show_archived_tasks { 1 } else { 0 })
        .bind(settings.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct UserSettingsRow {
    monday_target_minutes: i32,
    tuesday_target_minutes: i32,
    wednesday_target_minutes: i32,
    thursday_target_minutes: i32,
    fridays_target_minutes: i32,
    saturdays_target_minutes: i32,
    sundays_target_minutes: i32,
    show_archived_projects: i32,
    show_archived_tasks: i32,
    created_at: String,
    updated_at: String,
}

impl UserSettingsRow {
    pub fn parse(&self) -> anyhow::Result<UserSettings> {
        Ok(UserSettings {
            monday_target_minutes: self.monday_target_minutes as u32,
            tuesday_target_minutes: self.tuesday_target_minutes as u32,
            wednesday_target_minutes: self.wednesday_target_minutes as u32,
            thursday_target_minutes: self.thursday_target_minutes as u32,
            friday_target_minutes: self.fridays_target_minutes as u32,
            saturday_target_minutes: self.saturdays_target_minutes as u32,
            sunday_target_minutes: self.sundays_target_minutes as u32,
            show_archived_projects: self.show_archived_projects != 0,
            show_archived_tasks: self.show_archived_tasks != 0,
            created_at: DateTime::parse_from_rfc3339(&self.created_at)?.with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&self.updated_at)?.with_timezone(&Utc),
        })
    }
}
