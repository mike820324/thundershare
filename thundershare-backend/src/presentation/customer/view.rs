use crate::domain::entity::identity::Identity;
use crate::domain::service::customer::CustomerServiceTrait;
use crate::domain::service::ServerService;

use actix_web::cookie::time::{OffsetDateTime, Duration};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::Responder;
use actix_web::{web, HttpRequest, HttpResponse};
use super::dto::{CustomerSigninV1ReqDTO, CustomerSignupV1ReqDTO};

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

pub async fn customer_signup_v1 (
    server_services: web::Data<ServerService>,
    user_data: web::Json<CustomerSignupV1ReqDTO>,
) -> impl Responder {
    let svc = server_services.customer_service.clone();
    let result = svc.customer_signup(&user_data.username, &user_data.password).await;

    let token = result.unwrap().to_string().unwrap();
    let cookie = new_cookie(&token);
    HttpResponse::Ok().cookie(cookie).finish()
}

pub async fn customer_signin_v1 (
    server_services: web::Data<ServerService>,
    user_data: web::Json<CustomerSigninV1ReqDTO>
) -> impl Responder {
    let svc = server_services.customer_service.clone();
    let svc_result = svc.customer_signin(&user_data.username, &user_data.password).await;

    let token = svc_result.unwrap().to_string().unwrap();
    let cookie = new_cookie(&token);

    HttpResponse::Ok().cookie(cookie).finish()
}

pub async fn customer_signout_v1 (
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
