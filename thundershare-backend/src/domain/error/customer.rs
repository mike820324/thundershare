use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CustomerError {
    #[error("the customer is already register")]
    CustomerAlreadyExist,

    #[error("invalid username/password combination ")]
    CustomerInvalidCredential,
}
