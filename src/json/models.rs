use crate::utils::gen_random_hex;
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: String,
    pub label: String,
    pub content: String,
    pub created_timestamp: i64,
    pub updated_timestamp: i64,
}

impl Todo {
    pub fn new(label: &str, content: &str) -> (Self, String) {
        let id = gen_random_hex(8);
        let created_timestamp = Local::now().timestamp();
        let updated_timestamp = Local::now().timestamp();
        (
            Todo {
                id: id.clone(),
                label: String::from(label),
                content: String::from(content),
                created_timestamp,
                updated_timestamp,
            },
            id,
        )
    }
}
