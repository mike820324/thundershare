use crate::domain::entity::identity::Identity;
use crate::domain::error::file::FileError;
use crate::domain::service::file::FileServiceTrait;
use crate::domain::service::ServerService;
use crate::presentation::ResponseData;

use actix_multipart::form::MultipartForm;
use actix_web::Responder;
use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;
use log::info;

use super::dto::{map_domain_error_to_response, FileListByCustomerIdV1RespDTO, FileReadByIdV1RespDTO, FileUploadV1ReqDTO, FileUploadV1RespDTO};

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

            map_domain_error_to_response(domain_err, resp)
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

    info!("[DEBUG] {}", &token.value());
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
            let resp: ResponseData<FileReadByIdV1RespDTO> = domain_err.clone().into();
            map_domain_error_to_response(domain_err, resp)
        }
    }
}

pub async fn file_upload_v1(
    server_services: web::Data<ServerService>,
    MultipartForm(form): MultipartForm<FileUploadV1ReqDTO>,
    request: HttpRequest,
) -> impl Responder {
    // NOTE: authn checking
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

    let temp_filename = form.get_temp_filename();

    let svc = server_services.file_service.clone();
    let result = svc
        .file_upload(&identity.get_id(), &temp_filename)
        .await;

    match result {
        Ok(file_meta) => {
            let resp: ResponseData<FileUploadV1RespDTO> = file_meta.into();
            HttpResponse::Ok().json(resp)
        },
        Err(err) => {
            let domain_err: FileError = err.downcast().unwrap();
            let resp: ResponseData<FileUploadV1RespDTO> = domain_err.clone().into();

            map_domain_error_to_response(domain_err, resp)
        }
    }
}
