use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub name: String,
    pub age: i32
}