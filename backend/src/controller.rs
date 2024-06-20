use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {}

pub struct Todo {
    id: i32,
    title: String,
    description: String,
    done: bool,
}
