    use crate::integration_tests::test_db_builder;
    use crate::repositories::sqlite_project_repository::SQLiteProjectRepository;
    use chrono::{TimeZone, Utc};
    use domain::repositories::project_repository::ProjectRepository;
    use domain::types::Project;
    use uuid::Uuid;

    async fn setup_tests() -> SQLiteProjectRepository {
        let pool = test_db_builder::create_pool().await;
        SQLiteProjectRepository::new(pool)
    }

    #[tokio::test]
    async fn create_project_should_store_it() {
        let repository = setup_tests().await;

        let project = Project {
            id: Uuid::new_v4(),
            name: "ChronoForge".to_string(),
            time_limit: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        repository.create(project.clone()).await.unwrap();

        let stored_project = repository.find_by_id(&project.id).await.unwrap();
        assert!(stored_project.is_some());
        let stored_project = stored_project.unwrap();
        assert_eq!(project.id, stored_project.id);
        assert_eq!(project.name, stored_project.name);
    }

    #[tokio::test]
    async fn upsert_project_with_newer_version() {
        let repository = setup_tests().await;
        let project = Project {
            id: Uuid::new_v4(),
            name: "Chrono Forge".to_string(),
            time_limit: 0,
            created_at: Utc.with_ymd_and_hms(2026, 5, 27, 21, 5, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 5, 27, 21, 5, 0).unwrap(),
        };

        repository.create(project.clone()).await.unwrap();

        let mut updated_project = project.clone();
        updated_project.name = "ChronoForge".to_string();
        updated_project.time_limit = 20;
        updated_project.updated_at = Utc.with_ymd_and_hms(2026, 5, 27, 22, 5, 0).unwrap();

        repository.upsert(updated_project.clone()).await.unwrap();

        let stored_project = repository.find_by_id(&project.id).await.unwrap();
        assert!(stored_project.is_some());
        let stored_project = stored_project.unwrap();
        assert_eq!(updated_project.id, stored_project.id);
        assert_eq!(updated_project.name, stored_project.name);
        assert_eq!(updated_project.time_limit, stored_project.time_limit);
        assert_eq!(updated_project.updated_at, stored_project.updated_at);
    }
    #[tokio::test]
    async fn upsert_project_with_older_version() {
        let repository = setup_tests().await;
        let project = Project {
            id: Uuid::new_v4(),
            name: "Maze_Game".to_string(),
            time_limit: 0,
            created_at: Utc.with_ymd_and_hms(2026, 5, 27, 21, 5, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 5, 27, 21, 5, 0).unwrap(),
        };

        repository.create(project.clone()).await.unwrap();

        let mut updated_project = project.clone();
        updated_project.name = "Maze Game".to_string();
        updated_project.time_limit = 20;
        updated_project.updated_at = Utc.with_ymd_and_hms(2026, 5, 27, 20, 5, 0).unwrap();

        repository.upsert(updated_project.clone()).await.unwrap();

        let stored_project = repository.find_by_id(&project.id).await.unwrap();
        assert!(stored_project.is_some());
        let stored_project = stored_project.unwrap();
        assert_eq!(project.id, stored_project.id);
        assert_eq!(project.name, stored_project.name);
        assert_eq!(project.time_limit, stored_project.time_limit);
        assert_eq!(project.updated_at, stored_project.updated_at);
    }

    #[tokio::test]
    async fn find_by_id_should_return_none_when_project_doesnt_exist() {
        let repository = setup_tests().await;

        let project = Project {
            id: Uuid::new_v4(),
            name: "ChronoForge".to_string(),
            time_limit: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create(project.clone()).await.unwrap();

        let invalid_project_id = Uuid::new_v4();
        let stored_project = repository.find_by_id(&invalid_project_id).await.unwrap();
        assert!(stored_project.is_none());
    }

    #[tokio::test]
    async fn find_all_should_return_all_projects() {
        let repository = setup_tests().await;
        let project_a = Project {
            id: Uuid::new_v4(),
            name: "ChronoForge".to_string(),
            time_limit: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let project_b = Project {
            id: Uuid::new_v4(),
            name: "MazeGame".to_string(),
            time_limit: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let project_c = Project {
            id: Uuid::new_v4(),
            name: "JAREP".to_string(),
            time_limit: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create(project_a.clone()).await.unwrap();
        repository.create(project_b.clone()).await.unwrap();
        repository.create(project_c.clone()).await.unwrap();

        let stored_project = repository.find_all().await.unwrap();
        assert_eq!(stored_project.len(), 3);
        // The order here is by design. The database is supposed to sort the entries by name.
        // Therefore, projects need to be assigned manually to their respected original.
        assert_eq!(stored_project[0].id, project_a.id);
        assert_eq!(stored_project[0].name, project_a.name);
        assert_eq!(stored_project[2].id, project_b.id);
        assert_eq!(stored_project[2].name, project_b.name);
        assert_eq!(stored_project[1].id, project_c.id);
        assert_eq!(stored_project[1].name, project_c.name);
    }

    #[tokio::test]
    async fn find_all_should_return_empty_list_when_no_projects_exist() {
        let repository = setup_tests().await;
        let stored_project = repository.find_all().await.unwrap();
        assert_eq!(stored_project.len(), 0);
    }

    #[tokio::test]
    async fn update_should_update_existing_project() {
        let repository = setup_tests().await;
        let project_original = Project {
            id: Uuid::new_v4(),
            name: "ChronoForge".to_string(),
            time_limit: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        repository.create(project_original.clone()).await.unwrap();

        let updated_project = Project {
            id: project_original.id,
            name: "ZeitSchmiede".to_string(),
            time_limit: project_original.time_limit,
            created_at: project_original.created_at,
            updated_at: Utc::now(),
        };
        repository.update(updated_project.clone()).await.unwrap();
        let stored_project = repository.find_by_id(&project_original.id).await.unwrap();
        assert!(stored_project.is_some());
        let stored_project = stored_project.unwrap();
        assert_eq!(updated_project.id, stored_project.id);
        assert_eq!(updated_project.name, stored_project.name);
    }

    #[tokio::test]
    async fn delete_should_delete_existing_project() {
        let repository = setup_tests().await;
        let project = Project {
            id: Uuid::new_v4(),
            name: "ChronoForge".to_string(),
            time_limit: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        repository.create(project.clone()).await.unwrap();
        let stored_project = repository.find_by_id(&project.id).await.unwrap();
        assert!(stored_project.is_some());

        repository.delete(project.id).await.unwrap();
        let stored_project = repository.find_by_id(&project.id).await.unwrap();
        assert!(stored_project.is_none());
    }
