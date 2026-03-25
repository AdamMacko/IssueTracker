#[derive(Clone)]
pub struct Task {
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Clone)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}