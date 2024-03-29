use crate::domain::entity::identity::Identity;
use crate::domain::error::customer::CustomerError;
use crate::domain::service::customer::CustomerServiceTrait;
use crate::domain::service::ServerService;
use crate::presentation::ResponseData;

use super::dto::{CustomerGetByIdV1RespDTO, CustomerSigninV1ReqDTO, CustomerSignupV1ReqDTO, CustomerSignupV1RespDTO};
use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::Responder;
use actix_web::{web, HttpRequest, HttpResponse};

fn new_cookie(token: &str) -> Cookie {
    let mut now = OffsetDateTime::now_utc();
    now += Duration::days(180);

    let mut cookie = Cookie::new("token", token);
    cookie.set_path("/");
    cookie.set_expires(now);
    cookie.set_http_only(false);
    cookie.set_secure(true);
    cookie.set_same_site(SameSite::None);

    cookie
}

pub async fn customer_signup_v1(
    server_services: web::Data<ServerService>,
    user_data: web::Json<CustomerSignupV1ReqDTO>,
) -> impl Responder {
    let svc = server_services.customer_service.clone();
    let result = svc
        .customer_signup(&user_data.username, &user_data.password)
        .await;

    match result {
        Ok(identity) => {
            let resp: ResponseData<CustomerSignupV1RespDTO> = identity.clone().into();
            let token = identity.to_string().unwrap();
            let cookie = new_cookie(&token);
            HttpResponse::Created().cookie(cookie).json(resp)
        }
        Err(err) => {
            let domain_error: CustomerError = err.downcast().unwrap();
            let resp: ResponseData<CustomerSignupV1RespDTO> = domain_error.into();
            HttpResponse::BadRequest().json(resp)
        }
    }
}

pub async fn customer_signin_v1(
    server_services: web::Data<ServerService>,
    user_data: web::Json<CustomerSigninV1ReqDTO>,
) -> impl Responder {
    let svc = server_services.customer_service.clone();
    let svc_result = svc
        .customer_signin(&user_data.username, &user_data.password)
        .await;

    match svc_result {
        Ok(identity) => {
            let resp: ResponseData<CustomerSignupV1RespDTO> = identity.clone().into();
            let token = identity.to_string().unwrap();
            let cookie = new_cookie(&token);
            HttpResponse::Created().cookie(cookie).json(resp)
        }
        Err(err) => {
            let domain_error: CustomerError = err.downcast().unwrap();
            let resp: ResponseData<CustomerSignupV1RespDTO> = domain_error.into();
            HttpResponse::Unauthorized().json(resp)
        }
    }
}

pub async fn customer_signout_v1(
    server_services: web::Data<ServerService>,
    request: HttpRequest,
) -> impl Responder {
    let token = match request.cookie("token") {
        Some(token) => token,
        None => {
            return HttpResponse::Unauthorized().finish();
        }
    };

    let svc = server_services.customer_service.clone();
    let identity = Identity::from_string(token.value()).unwrap();
    let result = svc.customer_signout(&identity).await;

    let cookie = new_cookie("");
    HttpResponse::Ok().cookie(cookie).finish()
}

pub async fn customer_get_by_id_v1(
    server_services: web::Data<ServerService>,
    request: HttpRequest,
    id: web::Path<String>,
) -> impl Responder {
    let user_id: &str = id.as_str();
    if user_id != "self" {
        return HttpResponse::BadRequest().finish();
    }

    let token = match request.cookie("token") {
        Some(token) => token,
        None => {
            return HttpResponse::Unauthorized().finish();
        }
    };

    let identity = match Identity::from_string(&token.value()) {
        Ok(identity) => identity,
        Err(_err) => {
            return HttpResponse::Unauthorized().finish();
        }
    };

    let svc = server_services.customer_service.clone();
    let result = svc.get_customer_by_id(&identity.get_id()).await;

    match result {
        Ok(file_meta) => {
            let resp: ResponseData<CustomerGetByIdV1RespDTO> = file_meta.into();
            HttpResponse::Ok().json(resp)
        },
        Err(err) => {
            let domain_err: CustomerError = err.downcast().unwrap();
            let resp: ResponseData<CustomerGetByIdV1RespDTO> = domain_err.clone().into();

            match domain_err {
                CustomerError::CustomerNotFound => HttpResponse::NotFound().json(resp),
                _ => HttpResponse::InternalServerError().json(resp),
            }
        }
    }
}

