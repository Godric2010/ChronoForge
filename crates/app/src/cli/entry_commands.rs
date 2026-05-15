use crate::app_context::AppContext;
use sqlx::types::chrono::{DateTime, TimeZone, Utc};
use sqlx::types::Uuid;

#[derive(clap::Subcommand)]
pub enum EntryCommands {
    #[command(about = "Create a new time entry")]
    Create {
        #[arg(help = "The task id this entry belongs to")]
        task_id: String,
        #[arg(help = "Start time - year")]
        start_year: String,
        #[arg(help = "Start time - month")]
        start_month: String,
        #[arg(help = "Start time - day")]
        start_day: String,
        #[arg(help = "Start time - hour")]
        start_hour: String,
        #[arg(help = "Start time - minute")]
        start_minute: String,
        #[arg(help = "Stop time - year")]
        end_year: String,
        #[arg(help = "Stop time - month")]
        end_month: String,
        #[arg(help = "Stop time - day")]
        end_day: String,
        #[arg(help = "Stop time - hour")]
        end_hour: String,
        #[arg(help = "Stop time - minute")]
        end_minute: String,
    },
    #[command(about = "List all time entries of a task")]
    List {
        #[arg(help = "The id of the task")]
        task_id: String,
    },
    #[command(about = "Delete a time entry")]
    Delete {
        #[arg(help = "The time entry id to delete")]
        id: String,
    },
    #[command(about = "Re-assign a time entry to another task")]
    Assign {
        #[arg(help = "The time entry id to assign")]
        id: String,
        #[arg(help = "The task id to assign this entry to")]
        task_id: String,
    },
    #[command(about = "Edit a time entry")]
    Edit {
        #[arg(help = "The time entry id to edit")]
        id: String,
        #[arg(help = "Start time - year")]
        start_year: String,
        #[arg(help = "Start time - month")]
        start_month: String,
        #[arg(help = "Start time - day")]
        start_day: String,
        #[arg(help = "Start time - hour")]
        start_hour: String,
        #[arg(help = "Start time - minute")]
        start_minute: String,
        #[arg(help = "Stop time - year")]
        end_year: String,
        #[arg(help = "Stop time - month")]
        end_month: String,
        #[arg(help = "Stop time - day")]
        end_day: String,
        #[arg(help = "Stop time - hour")]
        end_hour: String,
        #[arg(help = "Stop time - minute")]
        end_minute: String,
    },
}

impl EntryCommands {
    pub async fn run(&self, app: &AppContext) -> anyhow::Result<()> {
        match self {
            EntryCommands::Create {
                task_id,
                start_year,
                start_month,
                start_day,
                start_hour,
                start_minute,
                end_year,
                end_month,
                end_day,
                end_hour,
                end_minute,
            } => {
                let task_uuid = Uuid::parse_str(task_id)?;
                let start =
                    self.to_utc(start_year, start_month, start_day, start_hour, start_minute);
                let end = self.to_utc(end_year, end_month, end_day, end_hour, end_minute);

                let entry = app
                    .time_entry_service
                    .create_manual(start, end, task_uuid)
                    .await?;
                println!(
                    "Created manual entry {} - {}",
                    entry.start_time, entry.end_time
                );
            }
            EntryCommands::List { task_id } => {
                let task_uuid = Uuid::parse_str(task_id)?;
                let task = app.task_service.find_by_id(task_uuid).await?;
                let entries = app
                    .time_entry_service
                    .find_all_entries_of_task(task_uuid)
                    .await?;
                println!("Entries of task {}", task.name);
                for entry in entries {
                    println!("\t{} - {}", entry.start_time, entry.end_time);
                }
            }
            EntryCommands::Delete { id } => {
                let uuid = Uuid::parse_str(id)?;
                app.time_entry_service.delete(uuid).await?;
                println!("Deleted entry {}", id);
            }
            EntryCommands::Assign { id, task_id } => {
                let uuid = Uuid::parse_str(id)?;
                let task_uuid = Uuid::parse_str(task_id)?;
                let task = app.task_service.find_by_id(uuid).await?;
                let entry = app
                    .time_entry_service
                    .assign_time_entry(uuid, task_uuid)
                    .await?;
                println!("Assigned entry {} to task {}", entry.id, task.name);
            }
            EntryCommands::Edit {
                id,
                start_year,
                start_month,
                start_day,
                start_hour,
                start_minute,
                end_year,
                end_month,
                end_day,
                end_hour,
                end_minute,
            } => {
                let uuid = Uuid::parse_str(id)?;
                let start =
                    self.to_utc(start_year, start_month, start_day, start_hour, start_minute);
                let end = self.to_utc(end_year, end_month, end_day, end_hour, end_minute);
                let entry = app
                    .time_entry_service
                    .edit_time_entry(uuid, start, end)
                    .await?;
                println!(
                    "Edited entry {} to {} - {}",
                    entry.id, entry.start_time, entry.end_time
                );
            }
        }
        Ok(())
    }

    fn to_utc(
        &self,
        year: &str,
        month: &str,
        day: &str,
        hour: &str,
        minute: &str,
    ) -> DateTime<Utc> {
        let year = year.parse::<i32>().unwrap();
        let month = month.parse::<u32>().unwrap();
        let day = day.parse::<u32>().unwrap();
        let hour = hour.parse::<u32>().unwrap();
        let minute = minute.parse::<u32>().unwrap();

        Utc.with_ymd_and_hms(year, month, day, hour, minute, 0)
            .unwrap()
    }
}
