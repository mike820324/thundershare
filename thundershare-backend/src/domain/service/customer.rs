use crate::domain::{entity::customer::Customer, repository::used_token::UsedTokenRepositoryTrait};
use crate::domain::error::customer::CustomerError;
use crate::domain::repository::customer::CustomerRepositoryTrait;
use anyhow::{bail, Result};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use mockall::automock;
use std::sync::Arc;
use tokio::sync::RwLock;

#[automock]
#[async_trait(?Send)]
pub trait CustomerServiceTrait {
    async fn customer_signup(&self, username: &str, password: &str) -> Result<Customer>;
    async fn customer_signin(&self, username: &str, password: &str) -> Result<Customer>;
    async fn customer_signout(&self, customer: &Customer) -> Result<()>;
    async fn get_customer_by_username(&self, username: &str) -> Result<Customer>;
}

pub struct CustomerServiceImpl {
    customer_repository: Arc<RwLock<dyn CustomerRepositoryTrait>>,
    used_token_repository: Arc<RwLock<dyn UsedTokenRepositoryTrait>>,
}

impl CustomerServiceImpl {
    pub fn new(
        customer_repository: Arc<RwLock<dyn CustomerRepositoryTrait>>,
        used_token_repository: Arc<RwLock<dyn UsedTokenRepositoryTrait>>,
    ) -> Arc<CustomerServiceImpl> {
        Arc::new(CustomerServiceImpl {
            customer_repository,
            used_token_repository,
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

        // TODO:
        // input should be a identity object that wrap up the customer ID
        // from the identity object, we can get the token and expire time
        let repo = self.used_token_repository.write().await;

        repo.create_used_token("", chrono::offset::Utc::now()).await?;
        Ok(())
    }

    async fn get_customer_by_username(&self, username: &str) -> Result<Customer> {
        let customer_list = {
            let repo = self.customer_repository.read().await;
            repo.get_customer_by_username(username).await?
        };

        if customer_list.len() == 0 {
            bail!(CustomerError::CustomerNotFound)
        }

        Ok(customer_list[0].clone())

    }
}
