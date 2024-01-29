use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use crate::domain::entity::file_meta::FileMeta;
use sqlx::types::Uuid;

#[automock]
#[async_trait]
pub trait FileMetaRepositoryTrait {
    async fn create(&self, url: &str) -> Result<FileMeta>;
    async fn get_file_meta_by_id(&self, id: &Uuid) -> Result<Vec<FileMeta>>;
    async fn list_file_meta_by_customer_id(&self, customer_id: &Uuid) -> Result<Vec<FileMeta>>;
}