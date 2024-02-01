use std::{sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::{entity::file_meta::FileSharingMeta, repository::file_sharing::FileSharingRepositoryTrait};

use super::DbPool;

#[derive(Debug, FromRow, Clone)]
struct FileSharingMetaDAO {
    id: Uuid,
    file_id: Uuid,
    link: String,
    expireat: DateTime<Utc>,
    password: Option<String>,
}

impl From<FileSharingMetaDAO> for FileSharingMeta {
    fn from(dao: FileSharingMetaDAO) -> FileSharingMeta {
        FileSharingMeta::new_full(
            &dao.id,
            &dao.file_id,
            &dao.link,
            &dao.expireat,
            &dao.password,
        )
    }
}

#[derive(Clone)]
pub struct FileSharingRepository {
    db_conn: DbPool,
}

impl FileSharingRepository {
    pub fn new(db_conn: DbPool) -> Arc<RwLock<dyn FileSharingRepositoryTrait>> {
        Arc::new(RwLock::new(FileSharingRepository { db_conn }))
    }
}

#[async_trait]
impl FileSharingRepositoryTrait for FileSharingRepository {
    async fn create(&self, file_id: &Uuid, link: &str, expireat: &DateTime<Utc>, password: &Option<String>) -> Result<FileSharingMeta> {
        let (id, ): (Uuid,) = sqlx::query_as(
            r#"
                INSERT INTO
                    filesharingmeta
                (file_id, link, expireat, password)
                VALUES
                    ($1, $2, $3, $4)
                RETURNING id;
            "#,
        )
        .bind(file_id)
        .bind(link)
        .bind(expireat)
        .bind(password)
        .fetch_one(&self.db_conn)
        .await?;

        let filemeta: FileSharingMetaDAO = sqlx::query_as(
            r#"
                SELECT id, file_id, link, expireat, password FROM
                    filesharingmeta
                WHERE
                    id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.db_conn)
        .await?;

        Ok(filemeta.into())

    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Vec<FileSharingMeta>> {
        let filemeta_list: Vec<FileSharingMeta> = sqlx::query_as(
            r#"
                SELECT id, file_id, link, expireat, password FROM
                    filesharingmeta
                WHERE
                    id = $1
            "#,
        )
        .bind(id)
        .fetch_all(&self.db_conn)
        .await?
        .into_iter()
        .map(|dao: FileSharingMetaDAO| dao.into())
        .collect();

        Ok(filemeta_list)
    }
}
