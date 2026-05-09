use crate::types::Project;
use crate::errors::AppError::ProjectNotFound;
use crate::errors::{AppError, AppResult};
use crate::repositories::project_repository::ProjectRepository;
use crate::services::naming_service;
use uuid::Uuid;

pub struct ProjectService<P: ProjectRepository> {
    project_repository: P,
}

impl<P: ProjectRepository> ProjectService<P> {
    pub fn new(project_repository: P) -> Self {
        Self { project_repository }
    }

    pub async fn create(&self, project_name: String) -> AppResult<Project> {
        if project_name.is_empty() {
            return Err(AppError::EmptyName);
        }

        let projects = self.project_repository.find_all().await?;
        let unique_project_name = self.create_unique_project_name(&project_name, &projects);

        let project = Project {
            id: Uuid::new_v4(),
            name: unique_project_name,
        };

        self.project_repository.create(project.clone()).await?;

        Ok(project)
    }

    pub async fn find_by_id(&self, project_id: Uuid) -> AppResult<Project> {
        if project_id == Uuid::default() || project_id == Uuid::nil() {
            return Err(ProjectNotFound);
        }

        let projects = self.project_repository.find_all().await?;
        let project = projects.iter().find(|p| p.id == project_id);
        if let Some(project) = project {
            Ok(project.clone())
        } else {
            Err(ProjectNotFound)
        }
    }

    pub async fn find_all(&self) -> AppResult<Vec<Project>> {
        let all_projects = self.project_repository.find_all().await?;
        Ok(all_projects)
    }

    pub async fn edit_name(&self, project_id: Uuid, name: &str) -> AppResult<Project> {
        if self.find_by_id(project_id).await.is_err() {
            return Err(ProjectNotFound);
        }
        if name.is_empty() {
            return Err(AppError::EmptyName);
        }

        let all_projects = self.project_repository.find_all().await?;
        let unique_project_name = self.create_unique_project_name(&name, &all_projects);

        let edited_project = Project {
            id: project_id,
            name: unique_project_name,
        };
        self.project_repository
            .update(edited_project.clone())
            .await?;
        Ok(edited_project)
    }

    pub async fn delete(&self, project_id: Uuid) -> AppResult<()> {
        if self.find_by_id(project_id).await.is_err() {
            return Err(ProjectNotFound);
        }

        self.project_repository.delete(project_id).await?;
        Ok(())
    }

    fn create_unique_project_name(&self, project_name: &str, projects: &[Project]) -> String {
        let mut names = Vec::new();
        for project in projects {
            names.push(project.name.clone());
        }

        let modified_name = naming_service::modify_name_with_count_of_equals(project_name, &names);
        modified_name
    }
}

#[cfg(test)]
mod project_service_tests {
    use super::*;
    use crate::errors::AppError;
    use crate::test_support::in_memory_project_repository::InMemoryProjectRepository;
    use uuid::Uuid;

    #[tokio::test]
    async fn create_new_project() {
        let project_repo = InMemoryProjectRepository::new();
        let service = ProjectService::new(project_repo);

        let result = service.create("MyProject".to_string()).await;
        assert!(result.is_ok());

        let all_projects_result = service.find_all().await;
        assert!(all_projects_result.is_ok());

        let all_projects = all_projects_result.unwrap();
        assert_eq!(all_projects.len(), 1);

        let test_project = all_projects.get(0).unwrap();
        assert_eq!(test_project.name, "MyProject");
        assert_ne!(test_project.id, Uuid::default());
    }

    #[tokio::test]
    async fn create_project_with_empty_name() {
        let project_repo = InMemoryProjectRepository::new();
        let service = ProjectService::new(project_repo);
        let result = service.create("".to_string()).await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), AppError::EmptyName));
    }

    #[tokio::test]
    async fn create_project_with_existing_name() {
        let project_repo = InMemoryProjectRepository::new();
        let service = ProjectService::new(project_repo);

        service.create("Jane Doe".to_string()).await.unwrap();
        service.create("Jane Doe".to_string()).await.unwrap();
        service.create("Jane Doe".to_string()).await.unwrap();

        let projects = service.find_all().await.unwrap();
        assert_eq!(projects.len(), 3);
        assert_eq!(projects.get(0).unwrap().name, "Jane Doe");
        assert_eq!(projects.get(1).unwrap().name, "Jane Doe(1)");
        assert_eq!(projects.get(2).unwrap().name, "Jane Doe(2)");
    }

    #[tokio::test]
    async fn get_project_by_existing_id() {
        let project_repo = InMemoryProjectRepository::new();
        let service = ProjectService::new(project_repo);
        let result = service.create("Jane Doe".to_string()).await;
        assert!(result.is_ok());

        let projects = service.find_all().await.unwrap();
        assert_eq!(projects.len(), 1);
        let project_id = projects.get(0).unwrap().id;

        let result = service.find_by_id(project_id).await;
        assert!(result.is_ok());

        let project = result.unwrap();
        assert_eq!(project.id, project_id);
        assert_eq!(project.name, projects.get(0).unwrap().name);
    }

    #[tokio::test]
    async fn get_project_by_non_existing_id() {
        let project_repo = InMemoryProjectRepository::new();
        let service = ProjectService::new(project_repo);

        let invalid_id = Uuid::new_v4();
        let result = service.find_by_id(invalid_id).await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ProjectNotFound));
    }

    #[tokio::test]
    async fn get_project_by_invalid_id() {
        let project_repo = InMemoryProjectRepository::new();
        let service = ProjectService::new(project_repo);
        let invalid_id = Uuid::default();
        let result = service.find_by_id(invalid_id).await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ProjectNotFound));
    }

    #[tokio::test]
    async fn edit_project_name() {
        let project_repo = InMemoryProjectRepository::new();
        let service = ProjectService::new(project_repo);
        service.create("Jane Doe".to_string()).await.unwrap();
        let projects = service.find_all().await.unwrap();
        let project = projects.get(0).unwrap();

        let result = service.edit_name(project.id, "Batman").await;
        assert!(result.is_ok());

        let target = service.find_by_id(project.id).await.unwrap();
        assert_eq!(target.name, "Batman");
    }

    #[tokio::test]
    async fn edit_project_name_with_invalid_id() {
        let project_repo = InMemoryProjectRepository::new();
        let service = ProjectService::new(project_repo);

        let result = service.edit_name(Uuid::new_v4(), "Gummibaum").await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ProjectNotFound));
    }

    #[tokio::test]
    async fn edit_project_name_to_existing_name() {
        let project_repo = InMemoryProjectRepository::new();
        let service = ProjectService::new(project_repo);
        service.create("Jane Doe".to_string()).await.unwrap();
        service.create("MyProject".to_string()).await.unwrap();
        let projects = service.find_all().await.unwrap();
        let project = projects.get(0).unwrap();

        let result = service.edit_name(project.id, "MyProject").await;
        assert!(result.is_ok());
        let target = service.find_by_id(project.id).await.unwrap();
        assert_eq!(target.name, "MyProject(1)");
    }

    #[tokio::test]
    async fn edit_project_name_to_invalid_name() {
        let project_repo = InMemoryProjectRepository::new();
        let service = ProjectService::new(project_repo);
        service.create("Jane Doe".to_string()).await.unwrap();
        let projects = service.find_all().await.unwrap();
        let project = projects.get(0).unwrap();
        let result = service.edit_name(project.id, "").await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), AppError::EmptyName));
    }

    #[tokio::test]
    async fn delete_project_by_existing_id() {
        let project_repo = InMemoryProjectRepository::new();
        let service = ProjectService::new(project_repo);
        service.create("Jane Doe".to_string()).await.unwrap();
        let projects = service.find_all().await.unwrap();
        let project = projects.get(0).unwrap();

        let result = service.delete(project.id).await;
        assert!(result.is_ok());

        let target = service.find_by_id(project.id).await;
        assert!(target.is_err());
        assert!(matches!(target.err().unwrap(), ProjectNotFound));
    }

    #[tokio::test]
    async fn delete_project_by_non_existing_id() {
        let project_repo = InMemoryProjectRepository::new();
        let service = ProjectService::new(project_repo);

        let result = service.delete(Uuid::new_v4()).await;
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ProjectNotFound));
    }
}
