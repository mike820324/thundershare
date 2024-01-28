use crate::domain::entity::customer::Customer;
use crate::domain::error::customer::CustomerError;
use crate::domain::repository::customer::CustomerRepositoryTrait;
use anyhow::{bail, Result};
use async_trait::async_trait;
use mockall::automock;
use std::sync::Arc;
use tokio::sync::RwLock;

#[automock]
#[async_trait(?Send)]
pub trait CustomerServiceTrait {
    async fn customer_signup(&self, username: &str, password: &str) -> Result<Customer>;
    async fn customer_signin(&self, username: &str, password: &str) -> Result<Customer>;
    async fn customer_signout(&self, customer: &Customer) -> Result<()>;
}

pub struct CustomerServiceImpl {
    customer_repository: Arc<RwLock<dyn CustomerRepositoryTrait>>,
}

impl CustomerServiceImpl {
    pub fn new(
        customer_repo: Arc<RwLock<dyn CustomerRepositoryTrait>>,
    ) -> Arc<CustomerServiceImpl> {
        Arc::new(CustomerServiceImpl {
            customer_repository: customer_repo,
        })
    }
}

#[async_trait(?Send)]
impl CustomerServiceTrait for CustomerServiceImpl {
    async fn customer_signup(&self, username: &str, password: &str) -> Result<Customer> {
        let customer_list = {
            let repo = self.customer_repository.read().await;
            repo.get_customer_by_username(username).await?
        };

        if customer_list.len() != 0 {
            bail!(CustomerError::CustomerAlreadyExist)
        }

        let customer = {
            let repo = self.customer_repository.write().await;
            repo.create_customer(username, password).await?
        };

        Ok(customer)
    }

    async fn customer_signin(&self, username: &str, password: &str) -> Result<Customer> {
        let customer_list = {
            let repo = self.customer_repository.read().await;
            repo.get_customer_by_credential(username, password).await?
        };

        if customer_list.len() == 0 {
            bail!(CustomerError::CustomerInvalidCredential)
        }

        Ok(customer_list[0].clone())
    }

    async fn customer_signout(&self, customer: &Customer) -> Result<()> {
        Ok(())
    }
}
