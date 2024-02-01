
use actix_files::NamedFile;
use anyhow::{bail, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, FixedOffset, Utc};
use log::info;
use mockall::automock;
use std::{io::Bytes, sync::Arc};
use tokio::{fs::rename, sync::RwLock};
use sqlx::types::Uuid;

use crate::domain::{entity::file_meta::{FileMeta, FileSharingMeta}, error::file::FileError, repository::{file_meta::FileMetaRepositoryTrait, file_sharing::FileSharingRepositoryTrait}};

#[automock]
#[async_trait(?Send)]
pub trait FileUploaderTrait {
    async fn upload(&self, src_filename: &str, dest_filename: &str) -> Result<()>;
    async fn download(&self, file_meta: &FileMeta) -> Result<NamedFile>;
}

pub struct LocalFileUploaderImpl {}

impl LocalFileUploaderImpl {
    pub fn new() -> Arc<dyn FileUploaderTrait> {
        Arc::new(LocalFileUploaderImpl{})
    }
}

#[async_trait(?Send)]
impl FileUploaderTrait for LocalFileUploaderImpl {
    async fn upload(&self, src_filename: &str, dest_filename: &str) -> Result<()> {
        info!("[DEBUG] src: {:?}, dest: {:?}", src_filename, dest_filename);
        rename(src_filename, dest_filename).await?;
        Ok(())
    }

    async fn download(&self, file_meta: &FileMeta) -> Result<NamedFile> {
        info!("[DEBUG] file meta: {:?}", file_meta);
        match NamedFile::open_async(file_meta.get_url()).await {
            Ok(file) => {
                Ok(file)
            }
            Err(_) => bail!(FileError::FileNotFound),
        }
    }
}

#[automock]
#[async_trait(?Send)]
pub trait FileServiceTrait {
    async fn file_upload(&self, customer_id: &Uuid, filename: &str) -> Result<FileMeta>;
    async fn file_read_by_id(&self, id: &Uuid, customer_id: &Uuid) -> Result<FileMeta>;
    async fn file_list_by_customer_id(&self, customer_id: &Uuid) -> Result<Vec<FileMeta>>;
    async fn file_create_sharing_link(&self, file_id: &Uuid, expireat: &DateTime<Utc>, password: &Option<String>) -> Result<FileSharingMeta>;
    async fn file_get_sharing_link_by_id(&self, file_id: &Uuid, password: Option<String>) -> Result<NamedFile>;
}


pub struct FileServiceImpl {
    curr_time_fn: Box<dyn Fn() -> DateTime<Utc>>,
    file_uploader: Arc<dyn FileUploaderTrait>,
    file_meta_repository: Arc<RwLock<dyn FileMetaRepositoryTrait>>,
    file_sharing_meta_repository: Arc<RwLock<dyn FileSharingRepositoryTrait>>,
}

impl FileServiceImpl {
    pub fn new(
        curr_time_fn: impl Fn() -> DateTime<Utc> + 'static,
        file_uploader: Arc<dyn FileUploaderTrait>,
        file_meta_repository: Arc<RwLock<dyn FileMetaRepositoryTrait>>,
        file_sharing_meta_repository: Arc<RwLock<dyn FileSharingRepositoryTrait>>,
    ) -> Arc<FileServiceImpl> {
        let svc = FileServiceImpl {
            curr_time_fn: Box::new(curr_time_fn),
            file_uploader: file_uploader.clone(),
            file_meta_repository: file_meta_repository.clone(),
            file_sharing_meta_repository: file_sharing_meta_repository.clone(),
        };

        Arc::new(svc)
    }

    fn fileid_generator(&self) -> String {
        Uuid::new_v4().to_string()
    }
}


#[async_trait(?Send)]
impl FileServiceTrait for FileServiceImpl {
    async fn file_upload(&self, customer_id: &Uuid, filename: &str) -> Result<FileMeta> {
        let dest_filename = self.fileid_generator();
        let url = dest_filename.clone();

        self.file_uploader.upload(filename, &dest_filename).await?;

        let file_meta = {
            let repo = self.file_meta_repository.write().await;
            repo.create(customer_id, &url).await?
        };
        Ok(file_meta)
    }

    async fn file_read_by_id(&self, id: &Uuid, customer_id: &Uuid) -> Result<FileMeta> {
        let file_meta_list = {
            let repo = self.file_meta_repository.read().await;
            repo.get_file_meta_by_id(id).await?
        };

        if file_meta_list.len() == 0 {
            bail!(FileError::FileNotFound)
        }

        let file_meta = file_meta_list[0].clone();
        if file_meta.get_customer_id() != *customer_id {
            bail!(FileError::FileNotBelongToCustomer)
        }


        Ok(file_meta)
    }

    async fn file_list_by_customer_id(&self, customer_id: &Uuid) -> Result<Vec<FileMeta>> {
        let file_meta_list = {
            let repo = self.file_meta_repository.read().await;
            repo.list_file_meta_by_customer_id(customer_id).await?
        };
        Ok(file_meta_list)
    }

    async fn file_create_sharing_link(&self, id: &Uuid, expireat: &DateTime<Utc>, password: &Option<String>) -> Result<FileSharingMeta> {
        let link = "TODO";
        let file_sharing_meta = {
            let repo = self.file_sharing_meta_repository.read().await;
            repo.create(id, link, expireat, password).await?
        };
        Ok(file_sharing_meta)
    }

    async fn file_get_sharing_link_by_id(&self, id: &Uuid, password: Option<String>) -> Result<NamedFile> {
        let file_sharing_meta_list = {
            let repo = self.file_sharing_meta_repository.read().await;
            repo.get_by_id(id).await?
        };

        if file_sharing_meta_list.len() == 0 {
            bail!(FileError::FileNotFound)
        }

        let curr_time = (self.curr_time_fn)();
        let file_sharing_meta = file_sharing_meta_list[0].clone();
        if (file_sharing_meta.is_expired(&curr_time)) {
            bail!(FileError::FileSharingLinkExpired)
        }

        if (!file_sharing_meta.is_password_correct(&password.unwrap_or(String::new()))) {
            bail!(FileError::FileSharingLinkPasswordIncorrect)
        }

        let file_meta_list = {
            let repo = self.file_meta_repository.read().await;
            repo.get_file_meta_by_id(&file_sharing_meta.get_file_id()).await?
        };

        if file_meta_list.len() == 0 {
            bail!(FileError::FileNotFound)
        }

        let file_meta = file_meta_list[0].clone();

        let file_stream = self.file_uploader.download(&file_meta).await?;
        Ok(file_stream)
    }

}