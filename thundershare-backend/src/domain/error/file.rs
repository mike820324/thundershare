use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum FileError {
    #[error("the requested file not exist")]
    FileNotFound,

    #[error("the requested file is not belong to customer")]
    FileNotBelongToCustomer,
}
