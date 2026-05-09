use crate::types::Task;
use crate::errors::{AppError, AppResult};
use crate::repositories::project_repository::ProjectRepository;
use crate::repositories::task_repository::TaskRepository;
use crate::services::naming_service;
use uuid::Uuid;

pub struct TaskService<T: TaskRepository, P: ProjectRepository> {
    task_repository: T,
    project_repository: P,
}

impl<T: TaskRepository, P: ProjectRepository> TaskService<T, P> {
    pub fn new(task_repository: T, project_repository: P) -> Self {
        Self {
            task_repository,
            project_repository,
        }
    }

    pub async fn create(&self, task_name: &str, project: &Uuid) -> AppResult<Task> {
        if task_name.is_empty() {
            return Err(AppError::EmptyName);
        }

        if self.project_repository.find_by_id(&project).await.is_err() {
            return Err(AppError::ProjectNotFound);
        }

        let tasks = self.task_repository.fina_all().await?;
        let unique_name = self.create_unique_task_name(task_name, &tasks, project.clone());

        let task = Task {
            id: Uuid::new_v4(),
            name: unique_name,
            project_id: project.clone(),
        };
        self.task_repository.create(task.clone()).await?;
        Ok(task)
    }

    pub async fn find_all(&self) -> AppResult<Vec<Task>> {
        let all_tasks = self.task_repository.fina_all().await?;
        Ok(all_tasks)
    }

    pub async fn find_by_id(&self, task_id: Uuid) -> AppResult<Task> {
        let tasks = self.task_repository.fina_all().await?;
        let task = tasks.into_iter().find(|task| task.id == task_id);
        if let Some(task) = task {
            return Ok(task);
        }
        Err(AppError::TaskNotFound)
    }

    pub async fn find_by_project_id(&self, project_id: Uuid) -> AppResult<Vec<Task>> {
        let tasks = self.task_repository.fina_all().await?;
        let mut matching_tasks = Vec::new();
        for task in tasks {
            if task.project_id == project_id {
                matching_tasks.push(task.clone());
            }
        }
        Ok(matching_tasks)
    }

    pub async fn edit_task_name(&self, task_id: Uuid, new_name: &str) -> AppResult<Task> {
        let tasks = self.task_repository.fina_all().await?;
        let task = self.find_by_id(task_id).await?;

        let unique_name = self.create_unique_task_name(new_name, &tasks, task.project_id);

        let new_task = Task {
            id: task.id,
            project_id: task.project_id,
            name: unique_name,
        };

        self.task_repository.update(new_task.clone()).await?;
        Ok(new_task)
    }

    pub async fn edit_task_project_relation(
        &self,
        task_id: Uuid,
        project: Uuid,
    ) -> AppResult<Task> {
        if self.project_repository.find_by_id(&project).await.is_err() {
            return Err(AppError::ProjectNotFound);
        }

        let tasks = self.task_repository.fina_all().await?;
        let task = self.find_by_id(task_id).await?;

        let unique_name = self.create_unique_task_name(&task.name, &tasks, project);

        let new_task = Task {
            id: task.id,
            project_id: project,
            name: unique_name,
        };

        self.task_repository.update(new_task.clone()).await?;
        Ok(new_task)
    }

    pub async fn delete(&self, task_id: Uuid) -> AppResult<()> {
        if self.find_by_id(task_id).await.is_err() {
            return Err(AppError::TaskNotFound);
        }
        self.task_repository.delete(task_id).await?;
        Ok(())
    }

    fn create_unique_task_name(&self, task_name: &str, tasks: &[Task], project_id: Uuid) -> String {
        let mut names = Vec::new();
        for task in tasks {
            if task.project_id != project_id {
                continue;
            }
            names.push(task.name.clone());
        }

        let modified_name = naming_service::modify_name_with_count_of_equals(task_name, &names);
        modified_name
    }
}

#[cfg(test)]
mod task_service_tests {
    use super::*;
    use crate::errors::AppError;
    use crate::services::project_service::ProjectService;
    use crate::test_support::in_memory_project_repository::InMemoryProjectRepository;
    use crate::test_support::in_memory_task_repository::InMemoryTaskRepository;

    struct Context {
        project_service: ProjectService<InMemoryProjectRepository>,
        task_service: TaskService<InMemoryTaskRepository, InMemoryProjectRepository>,
    }

    impl Context {
        pub fn new() -> Self {
            let project_repository = InMemoryProjectRepository::new();
            let project_service = ProjectService::new(project_repository.clone());
            let task_repository = InMemoryTaskRepository::new();
            let task_service =
                TaskService::new(task_repository.clone(), project_repository.clone());
            Self {
                project_service,
                task_service,
            }
        }
    }

    async fn create_project_and_get_id(context: &Context, name: &str) -> Option<Uuid> {

        let project = context.project_service.create(name.to_string()).await;
        if project.is_err() {
            return None;
        }
        let project = project.unwrap();
        Some(project.id)
    }

    #[tokio::test]
    async fn create_new_task() {
        let context = Context::new();
        let project_id = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id, None);
        let project_id = project_id.unwrap();

        let result = context.task_service.create("Task01", &project_id).await;
        assert!(result.is_ok());
        let task = result.unwrap();
        assert_eq!(task.project_id, project_id);
        assert_ne!(task.id, Uuid::default());
        assert_eq!(task.name, "Task01");
    }
    #[tokio::test]
    async fn create_task_with_invalid_name() {
        let context = Context::new();
        let project_id = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id, None);
        let project_id = project_id.unwrap();

        let result = context.task_service.create("", &project_id).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::EmptyName));
    }
    #[tokio::test]
    async fn create_task_with_invalid_project_id() {
        let context = Context::new();

        let result = context.task_service.create("Task01", &Uuid::new_v4()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ProjectNotFound));
    }
    #[tokio::test]
    async fn create_task_with_existing_name() {
        let context = Context::new();
        let project_id = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id, None);
        let project_id = project_id.unwrap();

        let mut tasks = vec![];
        tasks.push(
            context
                .task_service
                .create("Task", &project_id)
                .await
                .unwrap(),
        );
        tasks.push(
            context
                .task_service
                .create("Task", &project_id)
                .await
                .unwrap(),
        );
        tasks.push(
            context
                .task_service
                .create("Task", &project_id)
                .await
                .unwrap(),
        );

        assert_eq!(tasks.len(), 3);
        assert_eq!(tasks[0].name, "Task");
        assert_eq!(tasks[1].name, "Task(1)");
        assert_eq!(tasks[2].name, "Task(2)");
    }
    #[tokio::test]
    async fn create_task_with_existing_name_but_different_project() {
        let context = Context::new();
        let project_id_a = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id_a, None);
        let project_id_a = project_id_a.unwrap();

        let project_id_b = create_project_and_get_id(&context, "Project02").await;
        assert_ne!(project_id_b, None);
        let project_id_b = project_id_b.unwrap();

        let mut tasks = vec![];
        tasks.push(
            context
                .task_service
                .create("Task", &project_id_a)
                .await
                .unwrap(),
        );
        tasks.push(
            context
                .task_service
                .create("Task", &project_id_b)
                .await
                .unwrap(),
        );

        assert_eq!(tasks.len(), 2);
        // Since both tasks are parts of different projects, the names should remain in the original form.
        assert_eq!(tasks[0].name, "Task");
        assert_eq!(tasks[1].name, "Task");
    }

    #[tokio::test]
    async fn find_all_tasks() {
        let context = Context::new();
        let project_id = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id, None);
        let project_id = project_id.unwrap();

        context
            .task_service
            .create("Task 1", &project_id)
            .await
            .unwrap();
        context
            .task_service
            .create("Task 2", &project_id)
            .await
            .unwrap();
        context
            .task_service
            .create("Task 3", &project_id)
            .await
            .unwrap();
        context
            .task_service
            .create("Task 4", &project_id)
            .await
            .unwrap();
        context
            .task_service
            .create("Task 5", &project_id)
            .await
            .unwrap();

        let all_tasks = context.task_service.find_all().await;
        assert_eq!(all_tasks.is_ok(), true);
        let all_tasks = all_tasks.unwrap();
        assert_eq!(all_tasks.len(), 5);
    }
    #[tokio::test]
    async fn find_task_by_valid_id() {
        let context = Context::new();
        let project_id = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id, None);
        let project_id = project_id.unwrap();
        let task = context
            .task_service
            .create("Task 1", &project_id)
            .await
            .unwrap();

        let requested_task = context.task_service.find_by_id(task.id).await;
        assert!(requested_task.is_ok());
        let requested_task = requested_task.unwrap();
        assert_eq!(requested_task.id, task.id);
    }
    #[tokio::test]
    async fn find_task_by_invalid_id() {
        let context = Context::new();
        let project_id = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id, None);
        let project_id = project_id.unwrap();
        let _task = context
            .task_service
            .create("Task 1", &project_id)
            .await
            .unwrap();

        let requested_task = context.task_service.find_by_id(Uuid::new_v4()).await;
        assert!(requested_task.is_err());
        assert!(matches!(
            requested_task.unwrap_err(),
            AppError::TaskNotFound
        ));
    }
    #[tokio::test]
    async fn find_tasks_with_project_id() {
        let context = Context::new();
        let project_id_a = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id_a, None);
        let project_id_a = project_id_a.unwrap();

        let project_id_b = create_project_and_get_id(&context, "Project02").await;
        assert_ne!(project_id_b, None);
        let project_id_b = project_id_b.unwrap();

        context
            .task_service
            .create("Task 1", &project_id_a)
            .await
            .unwrap();
        context
            .task_service
            .create("Task 2", &project_id_b)
            .await
            .unwrap();
        context
            .task_service
            .create("Task 3", &project_id_a)
            .await
            .unwrap();

        let tasks_of_project_a = context.task_service.find_by_project_id(project_id_a).await;
        assert!(tasks_of_project_a.is_ok());
        let tasks_of_project_a = tasks_of_project_a.unwrap();
        assert_eq!(tasks_of_project_a.len(), 2);

        let tasks_of_project_b = context.task_service.find_by_project_id(project_id_b).await;
        assert!(tasks_of_project_b.is_ok());
        let tasks_of_project_b = tasks_of_project_b.unwrap();
        assert_eq!(tasks_of_project_b.len(), 1);

        assert_eq!(tasks_of_project_b[0].name, "Task 2");
    }

    #[tokio::test]
    async fn delete_task_with_valid_id() {
        let context = Context::new();
        let project_id = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id, None);
        let project_id = project_id.unwrap();

        let task = context.task_service.create("Task 1", &project_id).await;
        assert!(task.is_ok());
        let task = task.unwrap();

        let result = context.task_service.delete(task.id).await;
        assert!(result.is_ok());

        let find_by_id = context.task_service.find_by_id(task.id).await;
        assert!(find_by_id.is_err());
        assert!(matches!(find_by_id.unwrap_err(), AppError::TaskNotFound));
    }
    #[tokio::test]
    async fn delete_task_with_invalid_id() {
        let context = Context::new();
        let result = context.task_service.delete(Uuid::new_v4()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::TaskNotFound));
    }

    #[tokio::test]
    async fn edit_name_of_task_with_valid_id() {
        let context = Context::new();
        let project_id = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id, None);
        let project_id = project_id.unwrap();

        let task = context.task_service.create("Task 1", &project_id).await;
        assert!(task.is_ok());
        let task = task.unwrap();

        let modified_task = context
            .task_service
            .edit_task_name(task.id, "Task 10")
            .await;
        assert!(modified_task.is_ok());

        let modified_task = modified_task.unwrap();
        assert_eq!(modified_task.name, "Task 10");
        assert_eq!(modified_task.id, task.id);
    }

    #[tokio::test]
    async fn edit_name_of_task_with_invalid_id() {
        let context = Context::new();
        let modified_task = context
            .task_service
            .edit_task_name(Uuid::new_v4(), "Task 10")
            .await;
        assert!(modified_task.is_err());
        assert!(matches!(modified_task.unwrap_err(), AppError::TaskNotFound));
    }

    #[tokio::test]
    async fn edit_name_of_task_to_an_existing_name() {
        let context = Context::new();
        let project_id = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id, None);
        let project_id = project_id.unwrap();

        context
            .task_service
            .create("Task 1", &project_id)
            .await
            .unwrap();

        let task = context.task_service.create("Task 2", &project_id).await;
        assert!(task.is_ok());
        let task = task.unwrap();

        let modified_task = context.task_service.edit_task_name(task.id, "Task 1").await;
        assert!(modified_task.is_ok());

        let modified_task = modified_task.unwrap();
        assert_eq!(modified_task.name, "Task 1(1)");
        assert_eq!(modified_task.id, task.id);
    }

    #[tokio::test]
    async fn edit_task_project_id() {
        let context = Context::new();
        let project_id_a = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id_a, None);
        let project_id_a = project_id_a.unwrap();

        let project_id_b = create_project_and_get_id(&context, "Project02").await;
        assert_ne!(project_id_b, None);
        let project_id_b = project_id_b.unwrap();

        let task = context
            .task_service
            .create("Task 1", &project_id_a)
            .await
            .unwrap();

        let result = context
            .task_service
            .edit_task_project_relation(task.id, project_id_b)
            .await;
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.project_id, project_id_b);
        assert_eq!(result.name, task.name);
        assert_eq!(result.id, task.id);
    }

    #[tokio::test]
    async fn edit_task_project_id_with_invalid_project_id() {
        let context = Context::new();
        let project_id_a = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id_a, None);
        let project_id_a = project_id_a.unwrap();

        let task = context
            .task_service
            .create("Task 1", &project_id_a)
            .await
            .unwrap();

        let result = context
            .task_service
            .edit_task_project_relation(task.id, Uuid::new_v4())
            .await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ProjectNotFound));
    }

    #[tokio::test]
    async fn edit_task_with_project_id_task_name_doubling() {
        let context = Context::new();
        let project_id_a = create_project_and_get_id(&context, "Project01").await;
        assert_ne!(project_id_a, None);
        let project_id_a = project_id_a.unwrap();

        let project_id_b = create_project_and_get_id(&context, "Project02").await;
        assert_ne!(project_id_b, None);
        let project_id_b = project_id_b.unwrap();

        context
            .task_service
            .create("Task", &project_id_b)
            .await
            .unwrap();
        context
            .task_service
            .create("Task", &project_id_b)
            .await
            .unwrap();

        let task = context
            .task_service
            .create("Task", &project_id_a)
            .await
            .unwrap();

        let result = context
            .task_service
            .edit_task_project_relation(task.id, project_id_b)
            .await;
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.project_id, project_id_b);
        assert_eq!(result.name, "Task(2)");
        assert_eq!(result.id, task.id);
    }
}
