use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Debug, Clone)]
pub struct UserPayLoad {
    pub name: String,
    pub email: String,
}

impl UserPayLoad {
    fn new(name: String, email: String) -> Self {
        UserPayLoad { name, email }
    }
}
