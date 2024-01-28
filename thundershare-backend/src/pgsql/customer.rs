use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::domain::{entity::customer::Customer, repository::customer::CustomerRepositoryTrait};

use super::DbPool;


#[derive(Clone)]
pub struct CustomerRepository {
    db_conn: DbPool,
}

impl CustomerRepository {
    pub fn new(db_conn: DbPool) -> Arc<RwLock<CustomerRepository>> {
        Arc::new(RwLock::new(CustomerRepository { db_conn }))
    }
}

#[async_trait]
impl CustomerRepositoryTrait for CustomerRepository {
    async fn create_customer(&self, username: &str, password: &str) -> Result<Customer> {
        Ok(Customer::new("mikejiang"))
    }

    async fn get_customer_by_username(&self, username: &str) -> Result<Vec<Customer>> {
        Ok(vec![])
    }

    async fn get_customer_by_credential(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Vec<Customer>> {
        Ok(vec![])
    }
}

