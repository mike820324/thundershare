use uuid::Uuid;

use crate::{
    domain::{
        entity::{customer::Customer, identity::Identity},
        error::customer::CustomerError,
    },
    presentation::ResponseData,
};

#[derive(serde::Deserialize)]
pub struct CustomerSignupV1ReqDTO {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct CustomerSignupV1RespDTO {}

impl From<Identity> for ResponseData<CustomerSignupV1RespDTO> {
    fn from(svc_data: Identity) -> ResponseData<CustomerSignupV1RespDTO> {
        let resp = CustomerSignupV1RespDTO {};
        ResponseData::new(true, String::new(), Some(resp))
    }
}

impl From<CustomerError> for ResponseData<CustomerSignupV1RespDTO> {
    fn from(error: CustomerError) -> ResponseData<CustomerSignupV1RespDTO> {
        ResponseData::new(false, error.to_string(), None)
    }
}

#[derive(serde::Deserialize)]
pub struct CustomerSigninV1ReqDTO {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct CustomerSigninV1RespDTO {}

impl From<Identity> for ResponseData<CustomerSigninV1RespDTO> {
    fn from(svc_data: Identity) -> ResponseData<CustomerSigninV1RespDTO> {
        let resp = CustomerSigninV1RespDTO {};
        ResponseData::new(true, String::new(), Some(resp))
    }
}

impl From<CustomerError> for ResponseData<CustomerSigninV1RespDTO> {
    fn from(error: CustomerError) -> ResponseData<CustomerSigninV1RespDTO> {
        ResponseData::new(false, error.to_string(), None)
    }
}

#[derive(serde::Serialize)]
pub struct CustomerGetByIdV1RespDTO {
    id: Uuid,
    username: String,
}

impl From<Customer> for ResponseData<CustomerGetByIdV1RespDTO> {
    fn from(svc_data: Customer) -> ResponseData<CustomerGetByIdV1RespDTO> {
        let resp = CustomerGetByIdV1RespDTO {
            id: svc_data.get_id(),
            username: svc_data.get_username(),
        };

        ResponseData::new(true, String::new(), Some(resp))
    }
}

impl From<CustomerError> for ResponseData<CustomerGetByIdV1RespDTO> {
    fn from(error: CustomerError) -> ResponseData<CustomerGetByIdV1RespDTO> {
        ResponseData::new(false, error.to_string(), None)
    }
}