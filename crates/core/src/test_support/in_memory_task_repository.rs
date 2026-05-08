use crate::domain::Task;
use crate::errors::AppResult;
use crate::repositories::task_repository::TaskRepository;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct InMemoryTaskRepository {
    tasks: Arc<Mutex<Vec<Task>>>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl TaskRepository for InMemoryTaskRepository {
    async fn create(&self, task: Task) -> AppResult<()> {
        self.tasks.lock().unwrap().push(task);
        Ok(())
    }

    async fn update(&self, task: Task) -> AppResult<()> {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(existing_task) = tasks.iter_mut().find(|t| t.id == task.id) {
            *existing_task = task;
        }
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Task>> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks.iter().find(|t| t.id == id).cloned())
    }

    async fn find_by_project_id(&self, project_id: Uuid) -> AppResult<Vec<Task>> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks
            .iter()
            .filter(|t| t.project_id == project_id)
            .cloned()
            .collect())
    }

    async fn fina_all(&self) -> AppResult<Vec<Task>> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks.clone())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        self.tasks.lock().unwrap().retain(|t| t.id != id);
        Ok(())
    }
}
