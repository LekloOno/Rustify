use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("id '{id}' does not correspond to any registered node.")]
    NodeNotFound{id: usize},
    #[error("id '{id}' does not correpsond to any registered input.")]
    InputNotFound{id: usize},
}

pub type Result<T> = std::result::Result<T, Error>; 