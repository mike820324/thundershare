use std::sync::Arc;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{HttpResponse, Responder};
use uuid::Uuid;

use crate::{domain::{entity::file_meta::FileMeta, error::file::FileError}, presentation::ResponseData};

#[derive(serde::Serialize)]
pub struct FileReadByIdV1RespDTO {
    id: Uuid,
}

impl From<FileMeta> for ResponseData<FileReadByIdV1RespDTO> {
    fn from(data: FileMeta) -> ResponseData<FileReadByIdV1RespDTO> {
        let resp_data = Some(FileReadByIdV1RespDTO{id: data.get_id()});
        ResponseData::new(true, String::new(), resp_data)
    }
}

impl From<FileError> for ResponseData<FileReadByIdV1RespDTO> {
    fn from(error: FileError) -> ResponseData<FileReadByIdV1RespDTO> {
        ResponseData::new(false, error.to_string(), None)
    }
}

#[derive(serde::Serialize)]
struct FileMetaListItemV1RespDTO {
    id: Uuid,
}

impl From<FileMeta> for FileMetaListItemV1RespDTO {
    fn from(data: FileMeta) -> FileMetaListItemV1RespDTO {
        FileMetaListItemV1RespDTO {
            id: data.get_id()
        }
    }
}

#[derive(serde::Serialize)]
pub struct FileListByCustomerIdV1RespDTO {
    file_meta_list: Vec<FileMetaListItemV1RespDTO>,
}

impl From<Vec<FileMeta>> for ResponseData<FileListByCustomerIdV1RespDTO> {
    fn from(data: Vec<FileMeta>) -> ResponseData<FileListByCustomerIdV1RespDTO> {
        let file_meta_list: Vec<FileMetaListItemV1RespDTO> = data.into_iter()
        .map(|data| {
            data.into()
        })
        .collect();

        let resp_data = Some(FileListByCustomerIdV1RespDTO{file_meta_list: file_meta_list});
        ResponseData::new(true, String::new(), resp_data)
    }
}

#[derive(MultipartForm)]
pub struct FileUploadV1ReqDTO{
    #[multipart(limit = "32 MiB")]
    data: TempFile
}

impl FileUploadV1ReqDTO {
    pub fn get_temp_filename(&self) -> String {
        let temp_file = &self.data;
        temp_file.file.path().to_str().unwrap().to_string()
    }
}

#[derive(serde::Serialize)]
pub struct FileUploadV1RespDTO {
    id: Uuid,
}

impl From<FileMeta> for ResponseData<FileUploadV1RespDTO> {
    fn from(data: FileMeta) -> ResponseData<FileUploadV1RespDTO> {
        let resp_data = Some(FileUploadV1RespDTO{id: data.get_id()});
        ResponseData::new(true, String::new(), resp_data)
    }
}

impl From<FileError> for ResponseData<FileUploadV1RespDTO> {
    fn from(error: FileError) -> ResponseData<FileUploadV1RespDTO> {
        ResponseData::new(false, error.to_string(), None)
    }
}
pub fn map_domain_error_to_response<T: serde::Serialize>(err: FileError, resp: ResponseData<T>) -> HttpResponse {
    match err {
        FileError::FileNotFound => HttpResponse::NotFound().json(resp),
        FileError::FileNotBelongToCustomer => HttpResponse::Forbidden().json(resp),
        FileError::FileSharingLinkExpired => HttpResponse::Forbidden().json(resp),
        FileError::FileSharingLinkPasswordIncorrect => HttpResponse::Unauthorized().json(resp),
    }

}
