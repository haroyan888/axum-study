use thiserror::Error;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32)
}

pub struct Todo {
    id: i32,
    title: String,
    description: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: i32, title: String, description: String, completed: bool) -> Self {
        Self { id, title, description, completed }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CreateTodo {
    title: String,
    description: String
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateTodo {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>
}

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

pub struct TodoRepositoryForDB {
    
}
