use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum CustomerError {
    #[error("the customer is already register")]
    CustomerAlreadyExist,

    #[error("invalid username/password combination ")]
    CustomerInvalidCredential,

    #[error("customer not found")]
    CustomerNotFound,
}
