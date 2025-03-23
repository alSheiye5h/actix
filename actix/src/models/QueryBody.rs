use serde::Deserialize;


#[derive(Deserialize)]
pub struct QueryBodyStruct {
    pub username: String,
    pub age: i32,
    pub rank: i32
}