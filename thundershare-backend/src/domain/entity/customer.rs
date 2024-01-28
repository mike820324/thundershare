use sqlx::types::Uuid;

#[derive(PartialEq, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Customer {
    id: Uuid,
    username: String,
}

impl Customer {
    pub fn new(username: &str) -> Customer {
        Customer {
            id: Uuid::default(),
            username: username.to_string(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }
}
