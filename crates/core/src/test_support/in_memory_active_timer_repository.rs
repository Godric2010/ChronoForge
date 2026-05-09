use crate::domain::ActiveTimer;
use crate::errors::AppError::{NoActiveTimer, TimerAlreadyRunning};
use crate::errors::AppResult;
use crate::repositories::active_timer_repository::ActiveTimerRepository;
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
    async fn set(&self, entry: ActiveTimer) -> AppResult<()> {
        let mut active_timer = self.active_timer.lock().unwrap();
        if active_timer.is_some() {
            return Err(TimerAlreadyRunning);
        }
        *active_timer = Some(entry);
        Ok(())
    }

    async fn remove(&self) -> AppResult<()> {
        let mut active_timer = self.active_timer.lock().unwrap();
        if active_timer.is_none() {
            return Err(NoActiveTimer);
        }
        *active_timer = None;
        Ok(())
    }

    async fn get_active_timer(&self) -> Option<ActiveTimer> {
        let active_timer = self.active_timer.lock().unwrap().clone();
        active_timer
    }
}
