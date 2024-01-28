use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use crate::domain::repository::used_token::UsedTokenRepositoryTrait;
use super::DbPool;


#[derive(Clone)]
pub struct UsedTokenRepository {
    db_conn: DbPool,
}

impl UsedTokenRepository {
    pub fn new(db_conn: DbPool) -> Arc<RwLock<UsedTokenRepository>> {
        Arc::new(RwLock::new(UsedTokenRepository { db_conn }))
    }
}

#[async_trait]
impl UsedTokenRepositoryTrait for UsedTokenRepository {
    async fn create_used_token(&self, token: &str, expire_time: DateTime<Utc>) -> Result<()> {
        Ok(())
    }
}

