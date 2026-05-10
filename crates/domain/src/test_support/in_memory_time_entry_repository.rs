use crate::types::TimeEntry;
use crate::repositories::time_entry_repository::TimeEntryRepository;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct InMemoryTimeEntryRepository {
    time_entries: Arc<Mutex<Vec<TimeEntry>>>,
}

impl InMemoryTimeEntryRepository {
    pub fn new() -> Self {
        Self {
            time_entries: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
#[async_trait]
impl TimeEntryRepository for InMemoryTimeEntryRepository {
    async fn create(&self, time_entry: TimeEntry) -> anyhow::Result<()> {
        self.time_entries.lock().unwrap().push(time_entry);
        Ok(())
    }

    async fn update(&self, time_entry: TimeEntry) -> anyhow::Result<()> {
        let mut time_entries = self.time_entries.lock().unwrap();
        if let Some(entry) = time_entries.iter_mut().find(|t| t.id == time_entry.id) {
            *entry = time_entry;
        }
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<TimeEntry>> {
        Ok(self
            .time_entries
            .lock()
            .unwrap()
            .iter()
            .find(|t| t.id == id)
            .cloned())
    }

    async fn find_by_task_id(&self, task_id: Uuid) -> anyhow::Result<Vec<TimeEntry>> {
        Ok(self
            .time_entries
            .lock()
            .unwrap()
            .iter()
            .filter(|t| t.task_id == task_id)
            .cloned()
            .collect())
    }

    async fn find_all(&self) -> anyhow::Result<Vec<TimeEntry>> {
        let time_entries = self.time_entries.lock().unwrap();
        Ok(time_entries.clone())
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        self.time_entries.lock().unwrap().retain(|t| t.id != id);
        Ok(())
    }
}
