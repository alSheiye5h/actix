use regex::Regex;
use std::error::Error;


pub async fn check_regex_username(username: String) -> Result<bool, Box<dyn Error>> {
        let regex = Regex::new(r"^[a-z0-9_]+$").unwrap();
        Ok(regex.is_match(&username))    
}