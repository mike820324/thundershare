use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::prelude::FromRow;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::{entity::customer::Customer, repository::customer::CustomerRepositoryTrait};

use super::DbPool;

#[derive(Debug, FromRow, Clone)]
struct CustomerDAO {
    id: Uuid,
    username: String,
}

impl From<CustomerDAO> for Customer {
    fn from(dao: CustomerDAO) -> Customer {
        Customer::new_with_id(&dao.id, &dao.username)
    }
}

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
        sqlx::query(
            r#"
                INSERT INTO
                    customer (username, password)
                VALUES
                    ($1, $2)
            "#,
        )
        .bind(username)
        .bind(password)
        .execute(&self.db_conn)
        .await?;

        let customer: CustomerDAO = sqlx::query_as(
            r#"
                SELECT id, username FROM
                    customer
                WHERE
                    username = $1
            "#,
        )
        .bind(username)
        .fetch_one(&self.db_conn)
        .await?;

        Ok(customer.into())
    }

    async fn get_customer_by_username(&self, username: &str) -> Result<Vec<Customer>> {
        let customer_list: Vec<CustomerDAO> = sqlx::query_as(
            r#"
                SELECT id, username FROM
                    customer
                WHERE
                    username = $1
            "#,
        )
        .bind(username)
        .fetch_all(&self.db_conn)
        .await?;

        Ok(customer_list.into_iter().map(|dao| dao.into()).collect())
    }

    async fn get_customer_by_id(&self, id: &Uuid) -> Result<Vec<Customer>> {
        let customer_list: Vec<CustomerDAO> = sqlx::query_as(
            r#"
                SELECT id, username FROM
                    customer
                WHERE
                    id = $1
            "#,
        )
        .bind(id)
        .fetch_all(&self.db_conn)
        .await?;

        Ok(customer_list.into_iter().map(|dao| dao.into()).collect())
    }

    async fn get_customer_by_credential(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Vec<Customer>> {
        let customer_list: Vec<CustomerDAO> = sqlx::query_as(
            r#"
                SELECT id, username FROM
                    customer
                WHERE
                    username = $1
                    AND
                    password = $2
            "#,
        )
        .bind(username)
        .bind(password)
        .fetch_all(&self.db_conn)
        .await?;

        Ok(customer_list.into_iter().map(|dao| dao.into()).collect())
    }
}
