use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::repositories::project_repository::ProjectRepository;
use crate::types::Project;

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
    async fn create(&self, project: Project) -> anyhow::Result<()> {
        self.projects.lock().unwrap().push(project);
        Ok(())
    }

    async fn upsert(&self, project: Project) -> anyhow::Result<()> {
        let projects = &mut self.projects.lock().unwrap();
        let mut insertion_index: Option<usize> = None;
        for (index, existing_project) in projects.iter().enumerate() {
            if existing_project.id == project.id {
                if existing_project.updated_at < project.updated_at {
                    insertion_index = Some(index);
                    break;
                }
                return Ok(());
            }
        }

        if let Some(index) = insertion_index {
            self.projects.lock().unwrap().insert(index, project);
        } else {
            self.projects.lock().unwrap().push(project);
        }

        Ok(())
    }

    async fn update(&self, project: Project) -> anyhow::Result<()> {
        let mut projects = self.projects.lock().unwrap();
        if let Some(existing) = projects.iter_mut().find(|p| p.id == project.id) {
            *existing = project;
        }
        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Project>> {
        let projects = self.projects.lock().unwrap();
        let result = projects.iter().find(|p| p.id == *id);
        Ok(result.cloned())
    }

    async fn find_all(&self, include_archived: bool) -> anyhow::Result<Vec<Project>> {
        let projects = self.projects.lock().unwrap();
        if include_archived {
            return Ok(projects.clone());
        }

        let mut unarchived = Vec::new();
        projects.iter().for_each(|p| {
            if !p.is_archived {
                unarchived.push(p.clone());
            }
        });

        Ok(unarchived)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        self.projects.lock().unwrap().retain(|p| p.id != id);
        Ok(())
    }
}
