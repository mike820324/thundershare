use sqlx::types::Uuid;


#[derive(PartialEq, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FileMeta {
    id: Uuid,
    url: String,
}

impl FileMeta {
    pub fn new(url: &str) -> FileMeta {
        FileMeta {
            id: Uuid::default(),
            url: url.to_string(),
        }
    }

    pub fn new_with_id(id: &Uuid, url: &str) -> FileMeta {
        FileMeta {
            id: id.clone(),
            url: url.to_string(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}
