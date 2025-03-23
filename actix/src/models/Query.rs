use serde::Deserialize;


#[derive(Deserialize)]
pub struct QueryStruct {
    pub year: i32,
    pub method: String,
}