

pub mod showing;
pub mod user;
pub mod data;
pub mod username;
pub mod query;
pub mod request_handlers;

pub use showing::{hello, echo, manual_hello, index, get_app_name};
pub use user::get_user;
pub use data::user_data;
pub use username::check_username;
// pub use query::query_extract;  // will trigger error
// pub use query::query_body_extract;  // will trigger error
// pub use query::url_encoded;  // will trigger error
pub use request_handlers::return_static_str;
pub use request_handlers::return_string;
pub use request_handlers::return_bytes;
pub use request_handlers::get_RequestImplResponderObj;