pub mod customer;
#[cfg(test)]
pub mod customer_test;

pub mod file;

#[cfg(test)]
pub mod file_test;

use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

use self::{customer::CustomerServiceImpl, file::{FileServiceImpl, FileUploaderTrait}};

use super::repository::{customer::CustomerRepositoryTrait, file_meta::FileMetaRepositoryTrait, used_token::UsedTokenRepositoryTrait};

fn issue_at_fn() -> DateTime<Utc> {
    chrono::Utc::now()
}

#[derive(Clone)]
pub struct ServerService {
    pub customer_service: Arc<CustomerServiceImpl>,
    pub file_service: Arc<FileServiceImpl>,
}

impl ServerService {
    pub fn new(
        file_uploader: Arc<dyn FileUploaderTrait>,
        customer_repository: Arc<RwLock<dyn CustomerRepositoryTrait>>,
        used_token_repository: Arc<RwLock<dyn UsedTokenRepositoryTrait>>,
        file_meta_repository: Arc<RwLock<dyn FileMetaRepositoryTrait>>,
    ) -> ServerService {
        let customer_service =
            CustomerServiceImpl::new(issue_at_fn, customer_repository, used_token_repository);

        let file_service = FileServiceImpl::new(file_uploader, file_meta_repository);

        ServerService { customer_service, file_service }
    }
}
