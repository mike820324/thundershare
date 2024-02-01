use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(PartialEq, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FileSharingMeta {
    id: Uuid,
    file_id: Uuid,
    link: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    expireat: DateTime<Utc>,
    password: Option<String>,
}

impl FileSharingMeta {
    pub fn new_full(
        id: &Uuid,
        file_id: &Uuid,
        link: &str,
        expireat: &DateTime<Utc>,
        password: &Option<String>,
    ) -> FileSharingMeta {
        FileSharingMeta {
            id: id.clone(),
            file_id: file_id.clone(),
            link: link.to_string(),
            expireat: expireat.clone(),
            password: password.clone(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_link(&self) -> String {
        self.link.clone()
    }

    pub fn get_expireat(&self) -> DateTime<Utc> {
        self.expireat
    }

    pub fn is_expired(&self, curr_time: &DateTime<Utc>) -> bool {
        self.expireat < *curr_time
    }

    pub fn is_password_correct(&self, password: &str) -> bool {
        match self.password.clone() {
            Some(p) => p == password,
            None => true
        }
    }

}

#[derive(PartialEq, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FileMeta {
    id: Uuid,
    customer_id: Uuid,
    url: String,
}

impl FileMeta {
    pub fn new(url: &str) -> FileMeta {
        FileMeta {
            id: Uuid::default(),
            customer_id: Uuid::default(),
            url: url.to_string(),
        }
    }

    pub fn new_full(id: &Uuid, customer_id: &Uuid, url: &str) -> FileMeta {
        FileMeta {
            id: id.clone(),
            customer_id: customer_id.clone(),
            url: url.to_string(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_customer_id(&self) -> Uuid {
        self.customer_id
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}
