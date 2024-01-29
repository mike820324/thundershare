use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum FileError {
    #[error("the requested file not exist")]
    FileNotFound,
}
