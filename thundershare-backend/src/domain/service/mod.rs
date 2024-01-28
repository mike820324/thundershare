pub mod customer;
#[cfg(test)]
pub mod customer_test;

use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

use self::customer::CustomerServiceImpl;

use super::repository::{customer::CustomerRepositoryTrait, used_token::UsedTokenRepositoryTrait};

fn issue_at_fn() -> DateTime<Utc> {
    chrono::Utc::now()
}

#[derive(Clone)]
pub struct ServerService {
    pub customer_service: Arc<CustomerServiceImpl>,
}

impl ServerService {
    pub fn new(
        customer_repository: Arc<RwLock<dyn CustomerRepositoryTrait>>,
        used_token_repository: Arc<RwLock<dyn UsedTokenRepositoryTrait>>,
    ) -> ServerService {
        let customer_service =
            CustomerServiceImpl::new(issue_at_fn, customer_repository, used_token_repository);

        ServerService { customer_service }
    }
}
