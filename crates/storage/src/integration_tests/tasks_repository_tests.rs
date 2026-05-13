#[cfg(test)]
mod task_repository_tests {
    use crate::integration_tests::test_db_builder;
    use crate::repositories::sqlite_task_repository::SQLiteTaskRepository;
    use domain::repositories::task_repository::TaskRepository;
    use domain::types::Task;
    use sqlx::SqlitePool;
    use uuid::Uuid;

    async fn setup_test(project_one_id: Uuid, project_two_id: Uuid) -> SQLiteTaskRepository{
        let pool = test_db_builder::create_pool().await;
        test_db_builder::create_fake_project_entry(&pool, project_one_id, "Project1").await;
        test_db_builder::create_fake_project_entry(&pool, project_two_id, "Project2").await;
        let repository = SQLiteTaskRepository::new(pool.clone());
        repository
    }
    async fn create_test_project(pool: &SqlitePool, id: Uuid, name: &str) {
        sqlx::query(
            r#"
        INSERT INTO projects (id, name)
        VALUES (?, ?)
        "#,
        )
            .bind(id.to_string())
            .bind(name)
            .execute(pool)
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn create_project_and_store_it() {
        let project_id = Uuid::new_v4();
        let project_two_id = Uuid::new_v4();
        let repository = setup_test(project_id, project_two_id).await;

        let task = Task {
            id: Uuid::new_v4(),
            project_id: project_id.clone(),
            name: "TestTask01".to_string(),
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
    async fn update_project_and_store_it() {
        let project_one_id = Uuid::new_v4();
        let project_two_id = Uuid::new_v4();
        let repository = setup_test(project_one_id, project_two_id).await;

        let task = Task {
            id: Uuid::new_v4(),
            project_id: project_one_id.clone(),
            name: "TestTask01".to_string(),
        };
        repository.create(task.clone()).await.unwrap();

        let stored_task = repository.find_by_id(&task.id).await.unwrap();
        assert!(stored_task.is_some());

        let updated_name_task = Task {
            id: task.id,
            project_id: task.project_id,
            name: "TaskyMcTask".to_string(),
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
            project_id: project_two_id.clone(),
            name: updated_name_task.name,
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
            project_id: project_one_id.clone(),
            name: "TestTask01".to_string(),
        };
        repository.create(task_a.clone()).await.unwrap();

        let task_b = Task {
            id: Uuid::new_v4(),
            project_id: project_one_id.clone(),
            name: "TestTask02".to_string(),
        };
        repository.create(task_b.clone()).await.unwrap();

        let task_c = Task {
            id: Uuid::new_v4(),
            project_id: project_two_id.clone(),
            name: "TestTask01".to_string(),
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
            project_id: project_a.clone(),
            name: "TestTask01".to_string(),
        };
        repository.create(task_a.clone()).await.unwrap();

        let task_b = Task {
            id: Uuid::new_v4(),
            project_id: project_a.clone(),
            name: "TestTask02".to_string(),
        };
        repository.create(task_b.clone()).await.unwrap();

        let task_c = Task {
            id: Uuid::new_v4(),
            project_id: project_b.clone(),
            name: "TestTask01".to_string(),
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
            project_id: project_a.clone(),
            name: "TestTask01".to_string(),
        };
        repository.create(task_a.clone()).await.unwrap();

        let task_b = Task {
            id: Uuid::new_v4(),
            project_id: project_a.clone(),
            name: "TestTask02".to_string(),
        };
        repository.create(task_b.clone()).await.unwrap();

        let task_c = Task {
            id: Uuid::new_v4(),
            project_id: project_b.clone(),
            name: "TestTask01".to_string(),
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
