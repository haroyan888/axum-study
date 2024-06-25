use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String),
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct Todo {
    id: i32,
    title: String,
    description: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: i32, title: String, description: String, completed: bool) -> Self {
        Self {
            id,
            title,
            description,
            completed,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min=1, max=100))]
    title: String,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateTodo {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

#[async_trait]
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo>;
    async fn find(&self, id: i32) -> Result<Todo, RepositoryError>;
    async fn all(&self) -> anyhow::Result<Vec<Todo>>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo, RepositoryError>;
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct TodoRepositoryForDB {
    pool: SqlitePool,
}

impl TodoRepositoryForDB {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        Ok(TodoRepositoryForDB {
            pool: SqlitePool::connect(database_url).await?,
        })
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForDB {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"insert into todo(title, description) values ($1, $2) returning *"#,
        )
        .bind(payload.title.clone())
        .bind(payload.description.clone())
        .fetch_one(&self.pool)
        .await?;

        return Ok(todo);
    }

    async fn find(&self, id: i32) -> Result<Todo, RepositoryError> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
                select * from todo where id=?
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(todo)
    }

    async fn all(&self) -> anyhow::Result<Vec<Todo>> {
        let todos = sqlx::query_as::<_, Todo>(r#"select * from todo"#)
            .fetch_all(&self.pool)
            .await?;

        Ok(todos)
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo, RepositoryError> {
        let old_todo = self.find(id).await?;
        let todo =
            sqlx::query_as::<_, Todo>(r#"update todo set title=$1, description=$2, complete=$3"#)
                .bind(payload.title.unwrap_or(old_todo.title))
                .bind(payload.description.unwrap_or(old_todo.description))
                .bind(payload.completed.unwrap_or(old_todo.completed))
                .fetch_one(&self.pool)
                .await
                .map_err(|err| match err {
                    sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
                    _ => RepositoryError::Unexpected(err.to_string())
                })?;

        Ok(todo)
    }
    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        sqlx::query(r#"delete todo where id=$1"#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
                _ => RepositoryError::Unexpected(e.to_string()),
            })?;

        Ok(())
    }
}
