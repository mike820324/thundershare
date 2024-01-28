
#[derive(serde::Deserialize)]
pub struct CustomerSignupV1ReqDTO {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct CustomerSignupV1RespDTO {}

#[derive(serde::Deserialize)]
pub struct CustomerSigninV1ReqDTO {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct CustomerSigninV1RespDTO {}