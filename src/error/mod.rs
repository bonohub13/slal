#[derive(thiserror::Error, Debug, PartialEq)]
pub enum SlalError {
    #[error("Failed to initialize slal::matrix::Matrix: {0}")]
    MatrixInitializationError(String),
    #[error("Failed to update slal::matrix::Matrix: {0}")]
    MatrixUpdateError(String),
}

pub type SlalErr<V> = Result<V, SlalError>;
