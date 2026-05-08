use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::errors::AppError;
use crate::repositories::project_repository::ProjectRepository;
use crate::{domain::Project, errors::AppResult};

#[derive(Clone, Default)]
pub struct InMemoryProjectRepository {
    projects: Arc<Mutex<Vec<Project>>>,
}

impl InMemoryProjectRepository {
    pub fn new() -> Self {
        Self {
            projects: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl ProjectRepository for InMemoryProjectRepository {
    async fn create(&self, project: Project) -> AppResult<()> {
        self.projects.lock().unwrap().push(project);
        Ok(())
    }

    async fn update(&self, project: Project) -> AppResult<()> {
        let mut projects = self.projects.lock().unwrap();
        if let Some(existing) = projects.iter_mut().find(|p| p.id == project.id) {
            *existing = project;
        }
        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> AppResult<Project> {
        let projects = self.projects.lock().unwrap();
        let result = projects.iter().find(|p| p.id == id.clone());
        match result {
            Some(project) => Ok(project.clone()),
            None => Err(AppError::ProjectNotFound),
        }
    }

    async fn find_all(&self) -> AppResult<Vec<Project>> {
        let projects = self.projects.lock().unwrap();
        Ok(projects.clone())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        self.projects.lock().unwrap().retain(|p| p.id != id);
        Ok(())
    }
}
