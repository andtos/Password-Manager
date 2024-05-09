use serde::{Deserialize, Serialize};

use crate::storage::add_to_storage;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub struct Login;

impl Login {
    pub fn login(username: &str, password: &str) -> Option<User> {
        if username == "a" {
            return Some(User {
                username: username.to_string(),
                password: password.to_string(),
            });
        }
        None
    }

    pub fn register(username: &str, password: &str) -> Option<User> {
        let new_user = User {
            username: username.to_string(),
            password: password.to_string(),
        };
        match add_to_storage(username.to_string(), password.to_string()) {
            Ok(_) => Some(new_user),
            Err(_) => None,
        }
    }
}
