use crate::storage::{self, add_to_storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
}

pub struct Login;

impl Login {
    pub fn login(username: &str, password: &str) -> Option<User> {
        if storage::login(&username.to_string(), password.to_string()).is_some() {
            return Some(User {
                username: username.to_string(),
            });
        }
        None
    }

    pub fn register(username: &str, password: &str) -> Option<User> {
        let new_user = User {
            username: username.to_string(),
        };
        match add_to_storage(username.to_string(), password.to_string()) {
            Ok(_) => Some(new_user),
            Err(e) => {
                println!("{}", e.0);
                None
            }
        }
    }
}
