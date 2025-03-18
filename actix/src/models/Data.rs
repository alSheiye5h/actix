
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    pub name: String,
    pub password: String,
} 