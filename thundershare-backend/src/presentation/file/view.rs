use crate::domain::entity::identity::Identity;
use crate::domain::service::file::FileServiceTrait;
use crate::domain::service::ServerService;
use crate::presentation::ResponseData;

use actix_web::Responder;
use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use super::dto::{FileListByCustomerIdV1ReqDTO, FileReadByIdV1ReqDTO};

pub async fn file_read_by_id_v1(
    server_services: web::Data<ServerService>,
    request: HttpRequest,
    user_data: web::Json<FileReadByIdV1ReqDTO>,
) -> impl Responder {
    let token = match request.cookie("token") {
        Some(token) => token,
        None => {
            return HttpResponse::Unauthorized().finish();
        }
    };

    let identity = Identity::from_string(&token.value()).unwrap();

    let file_id = Uuid::default();

    let svc = server_services.file_service.clone();
    let result = svc.file_read_by_id(&file_id).await;

    HttpResponse::Ok().finish()
}

pub async fn file_list_by_customer_id_v1(
    server_services: web::Data<ServerService>,
    request: HttpRequest,
    user_data: web::Json<FileListByCustomerIdV1ReqDTO>,
) -> impl Responder {
    let token = match request.cookie("token") {
        Some(token) => token,
        None => {
            return HttpResponse::Unauthorized().finish();
        }
    };

    let identity = Identity::from_string(&token.value()).unwrap();

    let svc = server_services.file_service.clone();
    let svc_result = svc
        .file_list_by_customer_id(&identity.get_id())
        .await;

    HttpResponse::Ok().finish()
}

pub async fn file_upload_v1(
    server_services: web::Data<ServerService>,
    request: HttpRequest,
) -> impl Responder {
    let token = match request.cookie("token") {
        Some(token) => token,
        None => {
            return HttpResponse::Unauthorized().finish();
        }
    };

    let svc = server_services.file_service.clone();
    let svc_result = svc
        .file_upload(vec![])
        .await;

    HttpResponse::Ok().finish()
}
