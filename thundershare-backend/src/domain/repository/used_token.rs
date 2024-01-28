use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mockall::automock;

#[automock]
#[async_trait]
pub trait UsedTokenRepositoryTrait {
    async fn create_used_token(&self, token: &str, expire_time: DateTime<Utc>) -> Result<()>;
}
