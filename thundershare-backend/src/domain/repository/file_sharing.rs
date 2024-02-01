use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mockall::automock;
use crate::domain::entity::file_meta::{FileMeta, FileSharingMeta};
use sqlx::types::Uuid;

#[automock]
#[async_trait]
pub trait FileSharingRepositoryTrait {
    async fn create(&self, file_id: &Uuid, link: &str, expireat: &DateTime<Utc>, password: &Option<String>) -> Result<FileSharingMeta>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Vec<FileSharingMeta>>;
}