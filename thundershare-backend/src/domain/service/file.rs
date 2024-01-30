
use anyhow::{bail, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use mockall::automock;
use std::{io::Bytes, sync::Arc};
use tokio::{fs::rename, sync::RwLock};
use sqlx::types::Uuid;

use crate::domain::{entity::file_meta::FileMeta, error::file::FileError, repository::file_meta::FileMetaRepositoryTrait};

#[automock]
#[async_trait(?Send)]
pub trait FileUploaderTrait {
    async fn upload(&self, src_filename: &str, dest_filename: &str) -> Result<()>;
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
        rename(src_filename, dest_filename).await?;
        Ok(())
    }
}

#[automock]
#[async_trait(?Send)]
pub trait FileServiceTrait {
    async fn file_upload(&self, filename: &str) -> Result<FileMeta>;
    async fn file_read_by_id(&self, id: &Uuid, customer_id: &Uuid) -> Result<FileMeta>;
    async fn file_list_by_customer_id(&self, customer_id: &Uuid) -> Result<Vec<FileMeta>>;
}


pub struct FileServiceImpl {
    file_uploader: Arc<dyn FileUploaderTrait>,
    file_meta_repository: Arc<RwLock<dyn FileMetaRepositoryTrait>>,
}

impl FileServiceImpl {
    pub fn new(
        file_uploader: Arc<dyn FileUploaderTrait>,
        file_meta_repository: Arc<RwLock<dyn FileMetaRepositoryTrait>>,
    ) -> Arc<FileServiceImpl> {
        let svc = FileServiceImpl {
            file_uploader: file_uploader.clone(),
            file_meta_repository: file_meta_repository.clone(),
        };

        Arc::new(svc)
    }

    fn fileid_generator(&self) -> String {
        Uuid::new_v4().to_string()
    }
}


#[async_trait(?Send)]
impl FileServiceTrait for FileServiceImpl {
    async fn file_upload(&self, filename: &str) -> Result<FileMeta> {
        let dest_filename = self.fileid_generator();
        let url = dest_filename.clone();

        self.file_uploader.upload(filename, &dest_filename).await?;

        let file_meta = {
            let repo = self.file_meta_repository.write().await;
            repo.create(&url).await?
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

}