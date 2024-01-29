
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::prelude::FromRow;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::domain::{entity::file_meta::FileMeta, repository::file_meta::FileMetaRepositoryTrait};

use super::DbPool;

#[derive(Debug, FromRow, Clone)]
struct FileMetaDAO {
    id: Uuid,
    url: String,
}

impl From<FileMetaDAO> for FileMeta {
    fn from(dao: FileMetaDAO) -> FileMeta {
        FileMeta::new_with_id(&dao.id, &dao.url)
    }
}

#[derive(Clone)]
pub struct FileMetaRepository {
    db_conn: DbPool,
}

impl FileMetaRepository {
    pub fn new(db_conn: DbPool) -> Arc<RwLock<dyn FileMetaRepositoryTrait>> {
        Arc::new(RwLock::new(FileMetaRepository { db_conn }))
    }
}

#[async_trait]
impl FileMetaRepositoryTrait for FileMetaRepository {
    async fn create(&self, url: &str) -> Result<FileMeta> {
        Ok(FileMeta::new(""))
    }

    async fn get_file_meta_by_id(&self, id: &Uuid) -> Result<Vec<FileMeta>> {
        Ok(vec![])
    }

    async fn list_file_meta_by_customer_id(&self, customer_id: &Uuid) -> Result<Vec<FileMeta>> {
        Ok(vec![])
    }
}
