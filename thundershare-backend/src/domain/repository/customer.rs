use async_trait::async_trait;
use mockall::automock;
use anyhow::Result;

use crate::domain::entity::customer::Customer;


#[automock]
#[async_trait]
pub trait CustomerRepositoryTrait {
    async fn create_customer(&self, username: &str, password: &str) -> Result<Customer>;
    async fn get_customer_by_username(&self, username: &str) -> Result<Vec<Customer>>;
    async fn get_customer_by_credential(&self, username: &str, password: &str) -> Result<Vec<Customer>>;
}