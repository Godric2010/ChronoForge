use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task{
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
}