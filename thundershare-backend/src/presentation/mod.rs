pub mod customer;
pub mod file;

#[derive(serde::Serialize)]
pub struct ResponseData<T: serde::Serialize> {
    success: bool,
    error_msg: String,
    data: std::option::Option<T>,
}

impl<T: serde::Serialize> ResponseData<T> {
    pub fn new(success: bool, error_msg: String, data: Option<T>) -> ResponseData<T> {
        ResponseData {
            success,
            error_msg,
            data,
        }
    }
}