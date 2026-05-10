use crate::repositories::active_timer_repository::ActiveTimerRepository;
use crate::types::ActiveTimer;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct InMemoryActiveTimerRepository {
    active_timer: Arc<Mutex<Option<ActiveTimer>>>,
}

impl InMemoryActiveTimerRepository {
    pub fn new() -> Self {
        Self {
            active_timer: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ActiveTimerRepository for InMemoryActiveTimerRepository {
    async fn set(&self, entry: ActiveTimer) -> anyhow::Result<()> {
        let mut active_timer = self.active_timer.lock().unwrap();
        *active_timer = Some(entry);
        Ok(())
    }

    async fn remove(&self) -> anyhow::Result<()> {
        let mut active_timer = self.active_timer.lock().unwrap();
        *active_timer = None;
        Ok(())
    }

    async fn get_active_timer(&self) -> anyhow::Result<Option<ActiveTimer>> {
        let active_timer = self.active_timer.lock().unwrap().clone();
        Ok(active_timer)
    }
}
