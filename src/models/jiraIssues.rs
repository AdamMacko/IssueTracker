#[derive(Clone)]
pub enum Issue {
    epic(Epic),
    task,
    subtask
}

pub struct Epic{
    pub jiraId:Sting,
    pub name: String,
    pub tag: String,
    pub tasks:Vec<(Task)>
}

pub struct Task{
     pub jiraId:Sting,
    pub name: String,
    pub tag: String
}