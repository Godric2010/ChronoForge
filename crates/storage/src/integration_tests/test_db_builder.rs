use sqlx::{Pool, Sqlite, SqlitePool};
use uuid::Uuid;

pub async fn create_pool() -> Pool<Sqlite>{
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
    pool
}

pub async fn create_fake_project_entry(pool: &Pool<Sqlite>, project_id: Uuid, name: &str){
    sqlx::query(
        r#"
        INSERT INTO projects (id, name)
        VALUES (?, ?)
        "#,
    )
        .bind(project_id.to_string())
        .bind(name)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn create_fake_task_entry(pool: &Pool<Sqlite>, project_id: Uuid, task_id: Uuid, name: &str){
    sqlx::query(
        r#"
                    INSERT INTO tasks (id, project_id, name)
                    VALUES (?, ?, ?)
                    "#,
    )
        .bind(task_id.to_string())
        .bind(project_id.to_string())
        .bind(name)
        .execute(pool)
        .await
        .unwrap();
}