use crate::integration_tests::test_db_builder;
use crate::repositories::sqlite_active_timer_repository::SqliteActiveTimerRepository;
use chrono::{TimeZone, Utc};
use domain::repositories::active_timer_repository::ActiveTimerRepository;
use domain::types::ActiveTimer;
use uuid::Uuid;

async fn setup_test(project_id: Uuid, task_id: Uuid) -> SqliteActiveTimerRepository {
    let pool = test_db_builder::create_pool().await;
    test_db_builder::create_fake_project_entry(&pool, project_id, "Project1").await;
    test_db_builder::create_fake_task_entry(&pool, project_id, task_id, "Task1").await;
    SqliteActiveTimerRepository::new(pool)
}

#[tokio::test]
async fn set_active_timer_and_its_active() {
    let project_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();

    let repository = setup_test(project_id, task_id).await;

    let time_entry = ActiveTimer {
        task_id,
        start_time: Utc.with_ymd_and_hms(2026, 5, 24, 14, 00, 00).unwrap(),
    };

    let result = repository.set(time_entry.clone()).await;
    assert!(result.is_ok(), "{:?}", result);

    let active = repository.get_active_timer().await;
    assert!(active.is_ok());
    let active = active.unwrap();
    assert!(active.is_some());
    let active = active.unwrap();

    assert_eq!(time_entry.task_id, active.task_id);
    assert_eq!(time_entry.start_time, active.start_time);
}

#[tokio::test]
async fn remove_active_timer_and_its_not_active() {
    let project_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();

    let repository = setup_test(project_id, task_id).await;

    let time_entry = ActiveTimer {
        task_id,
        start_time: Utc.with_ymd_and_hms(2026, 5, 24, 14, 00, 00).unwrap(),
    };

    let result = repository.set(time_entry.clone()).await;
    assert!(result.is_ok(), "{:?}", result);

    let result = repository.remove().await;
    assert!(result.is_ok());

    let active = repository.get_active_timer().await;
    assert!(active.is_ok());
    let active = active.unwrap();
    assert!(active.is_none());
}
