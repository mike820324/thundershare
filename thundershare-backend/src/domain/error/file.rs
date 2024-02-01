use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum FileError {
    #[error("the requested file not exist")]
    FileNotFound,

    #[error("the requested file is not belong to customer")]
    FileNotBelongToCustomer,

    #[error("file sharing link is expired")]
    FileSharingLinkExpired,

    #[error("file sharing link passowrd incorrect")]
    FileSharingLinkPasswordIncorrect,
}
