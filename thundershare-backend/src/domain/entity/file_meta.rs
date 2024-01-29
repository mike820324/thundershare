use sqlx::types::Uuid;


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
