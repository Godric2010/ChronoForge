#[cfg(test)]
mod task_repository_tests {
    use crate::integration_tests::test_db_builder;
    use crate::repositories::sqlite_task_repository::SQLiteTaskRepository;
    use chrono::{TimeZone, Utc};
    use domain::repositories::task_repository::TaskRepository;
    use domain::types::Task;
    use uuid::Uuid;

    async fn setup_test(project_one_id: Uuid, project_two_id: Uuid) -> SQLiteTaskRepository {
        let pool = test_db_builder::create_pool().await;
        test_db_builder::create_fake_project_entry(&pool, project_one_id, "Project1").await;
        test_db_builder::create_fake_project_entry(&pool, project_two_id, "Project2").await;
        SQLiteTaskRepository::new(pool.clone())
    }

    #[tokio::test]
    async fn create_project_and_store_it() {
        let project_id = Uuid::new_v4();
        let project_two_id = Uuid::new_v4();
        let repository = setup_test(project_id, project_two_id).await;

        let task = Task {
            id: Uuid::new_v4(),
            project_id,
            name: "TestTask01".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create(task.clone()).await.unwrap();

        let stored_task = repository.find_by_id(&task.id).await.unwrap();
        assert!(stored_task.is_some());
        let stored_task = stored_task.unwrap();
        assert_eq!(stored_task.id, task.id);
        assert_eq!(stored_task.project_id, task.project_id);
        assert_eq!(stored_task.name, task.name);
    }
    #[tokio::test]
    async fn upsert_task_with_newer_version() {
        let project_id = Uuid::new_v4();
        let project_two_id = Uuid::new_v4();
        let repository = setup_test(project_id, project_two_id).await;
        let task = Task {
            id: Uuid::new_v4(),
            project_id,
            name: "Task 01".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc.with_ymd_and_hms(2026, 5, 27, 21, 5, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 5, 27, 21, 5, 0).unwrap(),
        };

        repository.create(task.clone()).await.unwrap();

        let mut updated_task = task.clone();
        updated_task.name = "Task01".to_string();
        updated_task.project_id = project_two_id;
        updated_task.time_limit = Some(20);
        updated_task.updated_at = Utc.with_ymd_and_hms(2026, 5, 27, 22, 5, 0).unwrap();

        repository.upsert(updated_task.clone()).await.unwrap();

        let stored_project = repository.find_by_id(&task.id).await.unwrap();
        assert!(stored_project.is_some());
        let stored_project = stored_project.unwrap();
        assert_eq!(updated_task.id, stored_project.id);
        assert_eq!(updated_task.name, stored_project.name);
        assert_eq!(updated_task.time_limit, stored_project.time_limit);
        assert_eq!(updated_task.updated_at, stored_project.updated_at);
    }
    #[tokio::test]
    async fn upsert_task_with_older_version() {
        let project_id = Uuid::new_v4();
        let project_two_id = Uuid::new_v4();
        let repository = setup_test(project_id, project_two_id).await;
        let task = Task {
            id: Uuid::new_v4(),
            project_id,
            name: "My new Task".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc.with_ymd_and_hms(2026, 5, 27, 21, 5, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 5, 27, 21, 5, 0).unwrap(),
        };

        repository.create(task.clone()).await.unwrap();

        let mut updated_task = task.clone();
        updated_task.name = "A new Task".to_string();
        updated_task.project_id = project_two_id;
        updated_task.time_limit = Some(20);
        updated_task.updated_at = Utc.with_ymd_and_hms(2026, 5, 27, 20, 5, 0).unwrap();

        repository.upsert(updated_task.clone()).await.unwrap();

        let stored_project = repository.find_by_id(&task.id).await.unwrap();
        assert!(stored_project.is_some());
        let stored_project = stored_project.unwrap();
        assert_eq!(task.id, stored_project.id);
        assert_eq!(task.name, stored_project.name);
        assert_eq!(task.time_limit, stored_project.time_limit);
        assert_eq!(task.updated_at, stored_project.updated_at);
    }
    #[tokio::test]
    async fn update_project_and_store_it() {
        let project_one_id = Uuid::new_v4();
        let project_two_id = Uuid::new_v4();
        let repository = setup_test(project_one_id, project_two_id).await;

        let task = Task {
            id: Uuid::new_v4(),
            project_id: project_one_id,
            name: "TestTask01".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create(task.clone()).await.unwrap();

        let stored_task = repository.find_by_id(&task.id).await.unwrap();
        assert!(stored_task.is_some());

        let updated_name_task = Task {
            id: task.id,
            project_id: task.project_id,
            name: "TaskyMcTask".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        repository.update(updated_name_task.clone()).await.unwrap();
        let stored_task = repository.find_by_id(&task.id).await.unwrap();
        assert!(stored_task.is_some());
        let stored_task = stored_task.unwrap();
        assert_eq!(stored_task.id, updated_name_task.id);
        assert_eq!(stored_task.project_id, updated_name_task.project_id);
        assert_eq!(stored_task.name, updated_name_task.name);

        let updated_project_task = Task {
            id: updated_name_task.id,
            project_id: project_two_id,
            name: updated_name_task.name,
            is_archived: false,
            time_limit: updated_name_task.time_limit,
            created_at: updated_name_task.created_at,
            updated_at: Utc::now(),
        };
        repository
            .update(updated_project_task.clone())
            .await
            .unwrap();
        let stored_task = repository.find_by_id(&task.id).await.unwrap();
        assert!(stored_task.is_some());
        let stored_task = stored_task.unwrap();
        assert_eq!(stored_task.id, updated_project_task.id);
        assert_eq!(stored_task.project_id, updated_project_task.project_id);
        assert_eq!(stored_task.name, updated_project_task.name);
    }

    #[tokio::test]
    async fn find_tasks_by_project_id() {
        let project_one_id = Uuid::new_v4();
        let project_two_id = Uuid::new_v4();
        let repository = setup_test(project_one_id, project_two_id).await;

        let task_a = Task {
            id: Uuid::new_v4(),
            project_id: project_one_id,
            name: "TestTask01".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create(task_a.clone()).await.unwrap();

        let task_b = Task {
            id: Uuid::new_v4(),
            project_id: project_one_id,
            name: "TestTask02".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create(task_b.clone()).await.unwrap();

        let task_c = Task {
            id: Uuid::new_v4(),
            project_id: project_two_id,
            name: "TestTask01".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create(task_c.clone()).await.unwrap();

        let result = repository.find_by_project_id(project_one_id).await.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, task_a.id);
        assert_eq!(result[0].project_id, project_one_id);
        assert_eq!(result[0].name, task_a.name);
        assert_eq!(result[1].id, task_b.id);
        assert_eq!(result[1].project_id, project_one_id);
        assert_eq!(result[1].name, task_b.name);
    }
    #[tokio::test]
    async fn find_all_tasks() {
        let project_a = Uuid::new_v4();
        let project_b = Uuid::new_v4();
        let repository = setup_test(project_a, project_b).await;

        let task_a = Task {
            id: Uuid::new_v4(),
            project_id: project_a,
            name: "TestTask01".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create(task_a.clone()).await.unwrap();

        let task_b = Task {
            id: Uuid::new_v4(),
            project_id: project_a,
            name: "TestTask02".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create(task_b.clone()).await.unwrap();

        let task_c = Task {
            id: Uuid::new_v4(),
            project_id: project_b,
            name: "TestTask01".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create(task_c.clone()).await.unwrap();

        let result = repository.find_all().await.unwrap();
        assert_eq!(result.len(), 3);
    }

    #[tokio::test]
    async fn delete_task_expect_it_to_be_gone() {
        let project_a = Uuid::new_v4();
        let project_b = Uuid::new_v4();
        let repository = setup_test(project_a, project_b).await;

        let task_a = Task {
            id: Uuid::new_v4(),
            project_id: project_a,
            name: "TestTask01".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create(task_a.clone()).await.unwrap();

        let task_b = Task {
            id: Uuid::new_v4(),
            project_id: project_a,
            name: "TestTask02".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create(task_b.clone()).await.unwrap();

        let task_c = Task {
            id: Uuid::new_v4(),
            project_id: project_b,
            name: "TestTask01".to_string(),
            time_limit: None,
            is_archived: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create(task_c.clone()).await.unwrap();

        let result = repository.delete(task_b.id).await;
        assert!(result.is_ok());

        let result = repository.find_by_id(&task_b.id).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_none());
    }
}
