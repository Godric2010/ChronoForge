use crate::repositories::task_repository::TaskRepository;
use crate::types::Task;
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
    async fn create(&self, task: Task) -> anyhow::Result<()> {
        self.tasks.lock().unwrap().push(task);
        Ok(())
    }

    async fn upsert(&self, task: Task) -> anyhow::Result<()> {
        let tasks = &mut self.tasks.lock().unwrap();
        let mut insertion_index: Option<usize> = None;
        for (index, existing_task) in tasks.iter().enumerate() {
            if existing_task.id == task.id {
                if existing_task.updated_at < task.updated_at {
                    insertion_index = Some(index);
                    break;
                }
                return Ok(());
            }
        }

        if let Some(index) = insertion_index {
            self.tasks.lock().unwrap().insert(index, task);
        } else {
            self.tasks.lock().unwrap().push(task);
        }

        Ok(())
    }

    async fn update(&self, task: Task) -> anyhow::Result<()> {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(existing_task) = tasks.iter_mut().find(|t| t.id == task.id) {
            *existing_task = task;
        }
        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Task>> {
        let tasks = self.tasks.lock().unwrap();
        let task = tasks.iter().find(|t| t.id == *id);
        Ok(task.cloned())
    }

    async fn find_by_project_id(
        &self,
        project_id: Uuid,
        include_archived: bool,
    ) -> anyhow::Result<Vec<Task>> {
        let tasks = self.tasks.lock().unwrap();
        if include_archived {
            return Ok(tasks
                .iter()
                .filter(|t| t.project_id == project_id)
                .cloned()
                .collect());
        }

        Ok(tasks
            .iter()
            .filter(|t| t.project_id == project_id && !t.is_archived)
            .cloned()
            .collect())
    }

    async fn find_all(&self, include_archived: bool) -> anyhow::Result<Vec<Task>> {
        let tasks = self.tasks.lock().unwrap();
        if include_archived {
            return Ok(tasks.clone());
        }
        let mut unarchived_tasks = Vec::new();
        tasks.iter().for_each(|t| {
            if !t.is_archived {
                unarchived_tasks.push(t.clone());
            }
        });
        Ok(unarchived_tasks)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        self.tasks.lock().unwrap().retain(|t| t.id != id);
        Ok(())
    }
}
