use crate::domain::entity::identity::Identity;
use crate::domain::error::file::FileError;
use crate::domain::service::file::FileServiceTrait;
use crate::domain::service::ServerService;
use crate::presentation::ResponseData;

use actix_web::Responder;
use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use super::dto::{FileListByCustomerIdV1RespDTO, FileReadByIdV1RespDTO, FileUploadV1RespDTO};

pub async fn file_read_by_id_v1(
    server_services: web::Data<ServerService>,
    request: HttpRequest,
    file_id: web::Path<Uuid>,
) -> impl Responder {
    // NOTE: authn checking
    let token = match request.cookie("token") {
        Some(token) => token,
        None => {
            return HttpResponse::Unauthorized().finish();
        }
    };

    // TODO: Add proper authz checking
    let identity = Identity::from_string(&token.value()).unwrap();

    let svc = server_services.file_service.clone();
    let result = svc.file_read_by_id(&file_id, &identity.get_id()).await;

    match result {
        Ok(file_meta) => {
            let resp: ResponseData<FileReadByIdV1RespDTO> = file_meta.into();
            HttpResponse::Ok().json(resp)
        },
        Err(err) => {
            let domain_err: FileError = err.downcast().unwrap();
            let resp: ResponseData<FileReadByIdV1RespDTO> = domain_err.clone().into();

            match domain_err {
                FileError::FileNotFound => HttpResponse::NotFound().json(resp),
                FileError::FileNotBelongToCustomer => HttpResponse::Forbidden().json(resp),
            }
        }
    }

}

pub async fn file_list_by_customer_id_v1(
    server_services: web::Data<ServerService>,
    request: HttpRequest,
) -> impl Responder {
    // NOTE: authn checking
    let token = match request.cookie("token") {
        Some(token) => token,
        None => {
            return HttpResponse::Unauthorized().finish();
        }
    };

    let identity = Identity::from_string(&token.value()).unwrap();
    let svc = server_services.file_service.clone();
    let result = svc
        .file_list_by_customer_id(&identity.get_id())
        .await;

    match result {
        Ok(file_meta_list) => {
            let resp: ResponseData<FileListByCustomerIdV1RespDTO> = file_meta_list.into();
            HttpResponse::Ok().json(resp)
        },
        Err(err) => {
            let domain_err: FileError = err.downcast().unwrap();
            let resp: ResponseData<FileReadByIdV1RespDTO> = domain_err.into();
            HttpResponse::InternalServerError().json(resp)
        }
    }
}

pub async fn file_upload_v1(
    server_services: web::Data<ServerService>,
    request: HttpRequest,
) -> impl Responder {
    // NOTE: authn checking
    let token = match request.cookie("token") {
        Some(token) => token,
        None => {
            return HttpResponse::Unauthorized().finish();
        }
    };

    let identity = Identity::from_string(&token.value()).unwrap();
    let svc = server_services.file_service.clone();
    let result = svc
        .file_upload(vec![])
        .await;

    match result {
        Ok(file_meta) => {
            let resp: ResponseData<FileUploadV1RespDTO> = file_meta.into();
            HttpResponse::Ok().json(resp)
        },
        Err(err) => {
            let domain_err: FileError = err.downcast().unwrap();
            let resp: ResponseData<FileUploadV1RespDTO> = domain_err.clone().into();

            match domain_err {
                FileError::FileNotFound => HttpResponse::NotFound().json(resp),
                FileError::FileNotBelongToCustomer => HttpResponse::Forbidden().json(resp),
            }
        }
    }
}
