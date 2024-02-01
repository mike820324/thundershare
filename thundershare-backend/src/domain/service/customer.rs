use crate::domain::entity::identity::Identity;
use crate::domain::error::customer::CustomerError;
use crate::domain::repository::customer::CustomerRepositoryTrait;
use crate::domain::{entity::customer::Customer, repository::used_token::UsedTokenRepositoryTrait};
use anyhow::{bail, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use mockall::automock;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[automock]
#[async_trait(?Send)]
pub trait CustomerServiceTrait {
    async fn customer_signup(&self, username: &str, password: &str) -> Result<Identity>;
    async fn customer_signin(&self, username: &str, password: &str) -> Result<Identity>;
    async fn customer_signout(&self, identity: &Identity) -> Result<()>;
    async fn get_customer_by_username(&self, username: &str) -> Result<Customer>;
    async fn get_customer_by_id(&self, username: &Uuid) -> Result<Customer>;
}

pub struct CustomerServiceImpl {
    issue_at_fn: Box<dyn Fn() -> DateTime<Utc>>,
    customer_repository: Arc<RwLock<dyn CustomerRepositoryTrait>>,
    used_token_repository: Arc<RwLock<dyn UsedTokenRepositoryTrait>>,
}

impl CustomerServiceImpl {
    pub fn new(
        issue_at_fn: impl Fn() -> DateTime<Utc> + 'static,
        customer_repository: Arc<RwLock<dyn CustomerRepositoryTrait>>,
        used_token_repository: Arc<RwLock<dyn UsedTokenRepositoryTrait>>,
    ) -> Arc<CustomerServiceImpl> {
        Arc::new(CustomerServiceImpl {
            issue_at_fn: Box::new(issue_at_fn),
            customer_repository,
            used_token_repository,
        })
    }
}

#[async_trait(?Send)]
impl CustomerServiceTrait for CustomerServiceImpl {
    async fn customer_signup(&self, username: &str, password: &str) -> Result<Identity> {
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

        let issueat = (self.issue_at_fn)();
        let duration = Duration::minutes(10);
        let identity = Identity::new(&customer, &issueat, duration);

        Ok(identity)
    }

    async fn customer_signin(&self, username: &str, password: &str) -> Result<Identity> {
        let customer_list = {
            let repo = self.customer_repository.read().await;
            repo.get_customer_by_credential(username, password).await?
        };

        if customer_list.len() == 0 {
            bail!(CustomerError::CustomerInvalidCredential)
        }

        let issueat = (self.issue_at_fn)();
        let duration = Duration::minutes(10);
        let identity = Identity::new(&customer_list[0], &issueat, duration);

        Ok(identity)
    }

    async fn customer_signout(&self, identity: &Identity) -> Result<()> {
        let repo = self.used_token_repository.write().await;
        repo.create_used_token(&identity.to_string()?, identity.get_expireat())
            .await?;
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

    async fn get_customer_by_id(&self, id: &Uuid) -> Result<Customer> {
        let customer_list = {
            let repo = self.customer_repository.read().await;
            repo.get_customer_by_id(id).await?
        };

        if customer_list.len() == 0 {
            bail!(CustomerError::CustomerNotFound)
        }

        Ok(customer_list[0].clone())
    }
}
