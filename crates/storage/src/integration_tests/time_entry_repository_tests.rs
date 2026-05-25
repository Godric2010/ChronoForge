mod time_entry_repository_tests {
    use crate::integration_tests::test_db_builder;
    use crate::repositories::sqlite_time_entry_repository::SqliteTimeEntryRepository;
    use chrono::{DateTime, TimeZone, Utc};
    use domain::repositories::time_entry_repository::TimeEntryRepository;
    use domain::types::TimeEntry;
    use uuid::Uuid;

    async fn setup_test(task_one_id: Uuid, task_two_id: Uuid) -> SqliteTimeEntryRepository {
        let pool = test_db_builder::create_pool().await;
        let project_id = Uuid::new_v4();
        test_db_builder::create_fake_project_entry(&pool, project_id.clone(), "Project1").await;
        test_db_builder::create_fake_task_entry(
            &pool,
            project_id.clone(),
            task_one_id.clone(),
            "Task1",
        )
        .await;
        test_db_builder::create_fake_task_entry(
            &pool,
            project_id.clone(),
            task_two_id.clone(),
            "Task2",
        )
        .await;
        SqliteTimeEntryRepository::new(pool)
    }

    fn utc_date_time(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
    }

    #[tokio::test]
    async fn create_time_entry_and_its_in_the_database() {
        let task_one_id = Uuid::new_v4();
        let task_two_id = Uuid::new_v4();
        let repository = setup_test(task_one_id.clone(), task_two_id.clone()).await;

        let time_entry = TimeEntry {
            id: Uuid::new_v4(),
            task_id: task_one_id,
            start_time: utc_date_time(2026, 05, 15),
            end_time: utc_date_time(2026, 05, 14),
            created_at: utc_date_time(2026, 05, 15),
            updated_at: utc_date_time(2026, 05, 14),
        };

        let result = repository.create(time_entry.clone()).await;
        assert!(result.is_ok());

        let stored = repository.find_by_id(time_entry.id).await;
        assert!(stored.is_ok());
        let stored = stored.unwrap();
        assert!(stored.is_some());
        let stored = stored.unwrap();
        assert_eq!(time_entry.id, stored.id);
        assert_eq!(time_entry.task_id, stored.task_id);
        assert_eq!(time_entry.start_time, stored.start_time);
        assert_eq!(time_entry.end_time, stored.end_time);
    }

    #[tokio::test]
    async fn update_time_entry_and_its_values_are_updated_in_the_database() {
        let task_one_id = Uuid::new_v4();
        let task_two_id = Uuid::new_v4();
        let repository = setup_test(task_one_id.clone(), task_two_id.clone()).await;

        let original = TimeEntry {
            id: Uuid::new_v4(),
            task_id: task_one_id,
            start_time: utc_date_time(2025, 12, 23),
            end_time: utc_date_time(2025, 12, 24),
            created_at: utc_date_time(2025, 12, 24),
            updated_at: utc_date_time(2025, 12, 24),
        };

        let result = repository.create(original.clone()).await;
        assert!(result.is_ok());

        let updated = TimeEntry {
            id: original.id,
            task_id: task_two_id,
            start_time: utc_date_time(2026, 04, 25),
            end_time: utc_date_time(2026, 04, 26),
            created_at: utc_date_time(2026, 04, 26),
            updated_at: utc_date_time(2026, 04, 26),
        };

        let result = repository.update(updated.clone()).await;
        assert!(result.is_ok());

        let stored = repository.find_by_id(original.id).await;
        assert!(stored.is_ok());
        let stored = stored.unwrap();
        assert!(stored.is_some());
        let stored = stored.unwrap();
        assert_eq!(updated.id, stored.id);
        assert_eq!(updated.task_id, stored.task_id);
        assert_eq!(updated.start_time, stored.start_time);
        assert_eq!(updated.end_time, stored.end_time);
    }

    #[tokio::test]
    async fn delete_time_entry_and_its_no_longer_in_the_database() {
        let task_one_id = Uuid::new_v4();
        let task_two_id = Uuid::new_v4();
        let repository = setup_test(task_one_id.clone(), task_two_id.clone()).await;

        let entry = TimeEntry {
            id: Uuid::new_v4(),
            task_id: task_two_id,
            start_time: utc_date_time(2012, 12, 12),
            end_time: utc_date_time(2012, 12, 16),
            created_at: utc_date_time(2012, 12, 16),
            updated_at: utc_date_time(2012, 12, 16),
        };
        let result = repository.create(entry.clone()).await;
        assert!(result.is_ok());

        let result = repository.delete(entry.id).await;
        assert!(result.is_ok());
        let stored = repository.find_by_id(entry.id).await;
        assert!(stored.is_ok());
        let stored = stored.unwrap();
        assert!(stored.is_none());
    }

    #[tokio::test]
    async fn get_entries_by_task_id() {
        let task_one_id = Uuid::new_v4();
        let task_two_id = Uuid::new_v4();
        let repository = setup_test(task_one_id.clone(), task_two_id.clone()).await;

        let one = TimeEntry {
            id: Uuid::new_v4(),
            task_id: task_one_id,
            start_time: utc_date_time(2025, 12, 23),
            end_time: utc_date_time(2025, 12, 24),
            created_at: utc_date_time(2025, 12, 24),
            updated_at: utc_date_time(2025, 12, 24),
        };
        let two = TimeEntry {
            id: Uuid::new_v4(),
            task_id: task_one_id,
            start_time: utc_date_time(2025, 8, 23),
            end_time: utc_date_time(2025, 9, 24),
            created_at: utc_date_time(2025, 9, 24),
            updated_at: utc_date_time(2025, 9, 24),
        };
        let three = TimeEntry {
            id: Uuid::new_v4(),
            task_id: task_two_id,
            start_time: utc_date_time(2025, 2, 23),
            end_time: utc_date_time(2025, 4, 24),
            created_at: utc_date_time(2025, 4,24 ),
            updated_at: utc_date_time(2025, 4, 24),
        };
        repository.create(one.clone()).await.unwrap();
        repository.create(two.clone()).await.unwrap();
        repository.create(three.clone()).await.unwrap();

        let entries_by_task = repository.find_by_task_id(task_one_id).await;
        assert!(entries_by_task.is_ok());
        let entries_by_task = entries_by_task.unwrap();
        assert_eq!(entries_by_task.len(), 2);
        assert_eq!(entries_by_task[0].id, one.id);
        assert_eq!(entries_by_task[1].id, two.id);
    }

    #[tokio::test]
    async fn get_all_entries() {
        let task_one_id = Uuid::new_v4();
        let task_two_id = Uuid::new_v4();
        let repository = setup_test(task_one_id.clone(), task_two_id.clone()).await;

        let one = TimeEntry {
            id: Uuid::new_v4(),
            task_id: task_one_id,
            start_time: utc_date_time(2025, 12, 23),
            end_time: utc_date_time(2025, 12, 24),
            created_at: utc_date_time(2025, 12, 24),
            updated_at: utc_date_time(2025, 12, 24),
        };
        let two = TimeEntry {
            id: Uuid::new_v4(),
            task_id: task_one_id,
            start_time: utc_date_time(2025, 8, 23),
            end_time: utc_date_time(2025, 9, 24),
            created_at: utc_date_time(2025, 9, 24),
            updated_at: utc_date_time(2025, 9, 24),
        };
        let three = TimeEntry {
            id: Uuid::new_v4(),
            task_id: task_two_id,
            start_time: utc_date_time(2025, 2, 23),
            end_time: utc_date_time(2025, 4, 24),
            created_at: utc_date_time(2025, 4, 24),
            updated_at: utc_date_time(2025, 4, 24),
        };
        repository.create(one.clone()).await.unwrap();
        repository.create(two.clone()).await.unwrap();
        repository.create(three.clone()).await.unwrap();

        let all_entries = repository.find_all().await;
        assert!(all_entries.is_ok());
        let all_entries = all_entries.unwrap();
        assert_eq!(all_entries.len(), 3);
        assert_eq!(all_entries[0].id, one.id);
        assert_eq!(all_entries[1].id, two.id);
        assert_eq!(all_entries[2].id, three.id);
    }
}
