
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
    customer_id: Uuid,
    url: String,
}

impl From<FileMetaDAO> for FileMeta {
    fn from(dao: FileMetaDAO) -> FileMeta {
        FileMeta::new_full(&dao.id, &dao.customer_id, &dao.url)
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
    async fn create(&self, customer_id: &Uuid, url: &str) -> Result<FileMeta> {
        let (id, ): (Uuid,) = sqlx::query_as(
            r#"
                INSERT INTO
                    filemeta (customer_id, url)
                VALUES
                    ($1, $2)
                RETURNING id;
            "#,
        )
        .bind(customer_id)
        .bind(url)
        .fetch_one(&self.db_conn)
        .await?;

        let filemeta: FileMetaDAO = sqlx::query_as(
            r#"
                SELECT id, customer_id, url FROM
                    filemeta
                WHERE
                    id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.db_conn)
        .await?;

        Ok(filemeta.into())
    }

    async fn get_file_meta_by_id(&self, id: &Uuid) -> Result<Vec<FileMeta>> {
        let filemeta: Vec<FileMeta> = sqlx::query_as(
            r#"
                SELECT id, customer_id, url FROM
                    filemeta
                WHERE
                    id = $1
            "#,
        )
        .bind(id)
        .fetch_all(&self.db_conn)
        .await?
        .into_iter()
        .map(|dao: FileMetaDAO| dao.into())
        .collect();

        Ok(filemeta)
    }

    async fn list_file_meta_by_customer_id(&self, customer_id: &Uuid) -> Result<Vec<FileMeta>> {
        let filemeta: Vec<FileMeta> = sqlx::query_as(
            r#"
                SELECT id, customer_id, url FROM
                    filemeta
                WHERE
                    customer_id = $1
            "#,
        )
        .bind(customer_id)
        .fetch_all(&self.db_conn)
        .await?
        .into_iter()
        .map(|dao: FileMetaDAO| dao.into())
        .collect();

        Ok(filemeta)
    }
}
