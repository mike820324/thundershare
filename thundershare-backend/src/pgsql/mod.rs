pub mod customer;
pub mod used_token;

pub type DbPool = sqlx::postgres::PgPool;
use std::sync::Arc;

use tokio::sync::RwLock;
use urlencoding::encode;

use crate::domain::repository::{
    customer::CustomerRepositoryTrait, used_token::UsedTokenRepositoryTrait,
};

use self::{customer::CustomerRepository, used_token::UsedTokenRepository};

pub async fn connection_builder() -> Result<DbPool, sqlx::Error> {
    let db_user = std::env::var("DB_USER").unwrap();
    let db_host = std::env::var("DB_HOST").unwrap();
    let db_name = std::env::var("DB_NAME").unwrap();

    let db_pass = std::env::var("DB_PASS").unwrap();
    let encoded_db_pass = encode(&db_pass);

    let connectspec = format!(
        "postgres://{}:{}@{}/{}",
        db_user, encoded_db_pass, db_host, db_name
    );

    sqlx::postgres::PgPool::connect(&connectspec).await
}

pub struct ServerRepositories {
    pub customer_repository: Arc<RwLock<dyn CustomerRepositoryTrait>>,
    pub used_token_repository: Arc<RwLock<dyn UsedTokenRepositoryTrait>>,
}

impl ServerRepositories {
    pub fn new(db_pool: DbPool) -> ServerRepositories {
        let customer_repository = CustomerRepository::new(db_pool.clone());
        let used_token_repository = UsedTokenRepository::new(db_pool);

        ServerRepositories {
            customer_repository,
            used_token_repository,
        }
    }
}
