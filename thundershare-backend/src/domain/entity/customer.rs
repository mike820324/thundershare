use sqlx::types::Uuid;

#[derive(PartialEq, Clone, Debug)]
pub struct Customer {
    id: Uuid,
    username: String,
    password: String,
}

impl Customer {
    pub fn new(username: &str, password: &str) -> Customer {
        Customer {
            id: Uuid::default(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }
}
