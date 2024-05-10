use std::{
    collections::{hash_map::Entry, HashMap},
    fs::{self, File, OpenOptions},
};

use serde::{Deserialize, Serialize};
pub const STORAGE_FILE: &str = "storage/storage.json";

pub struct StorageError(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginDetails {
    password: String,
    username: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    login_password: String,
    passwords: HashMap<String, LoginDetails>,
}

impl User {
    pub fn new(login_password: String) -> Self {
        User {
            login_password,
            passwords: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Storage {
    data: HashMap<String, User>,
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn load(path: &str) -> Result<Self, StorageError> {
        let file = File::open(path).map_err(|e| StorageError(e.to_string()))?;
        let deserialized: Storage =
            serde_json::from_reader(file).map_err(|e| StorageError(e.to_string()))?;
        Ok(deserialized)
    }
}

pub fn login(username: &String, password: String) -> Option<String> {
    if let Ok(storage) = Storage::load(STORAGE_FILE) {
        let user = storage.data.get(username)?;
        if user.login_password == password {
            return Some(username.to_string());
        }
    }
    None
}

pub fn add_site(username: &String, site_name: String, password: &str) -> Result<(), StorageError> {
    let mut storage = Storage::load(STORAGE_FILE)?;
    let user = storage.data.get_mut(username).unwrap();
    let login_info = LoginDetails {
        username: username.to_string(),
        password: password.to_string(),
    };
    user.passwords.insert(site_name, login_info);
    let json_string =
        serde_json::to_string_pretty(&storage).map_err(|e| StorageError(e.to_string()))?;
    fs::write(STORAGE_FILE, json_string.as_bytes()).map_err(|e| StorageError(e.to_string()))?;
    Ok(())
}

pub fn retrieve_password(username: &String, site_name: String) -> Option<(String, String)> {
    if let Ok(storage) = Storage::load(STORAGE_FILE) {
        let user = storage.data.get(username)?;
        let details = user.passwords.get(&site_name)?;
        Some((details.username.to_string(), details.password.to_string()))
    } else {
        None
    }
}

pub fn add_to_storage(username: String, password: String) -> Result<(), StorageError> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(STORAGE_FILE)
        .map_err(|e| StorageError(e.to_string()))?;
    let mut deserialized: Storage = serde_json::from_reader(&file).unwrap_or(Storage::new());
    let new_user = User::new(password);
    match deserialized.data.entry(username) {
        Entry::Occupied(_) => return Err(StorageError("Username is taken".to_string())),
        Entry::Vacant(entry) => {
            entry.insert(new_user);
            let json_string = serde_json::to_string_pretty(&deserialized)
                .map_err(|e| StorageError(e.to_string()))?;
            fs::write(STORAGE_FILE, json_string.as_bytes())
                .map_err(|e| StorageError(e.to_string()))?;
        }
    }

    Ok(())
}

pub fn create_new_storage() -> Result<(), std::io::Error> {
    fs::File::create(STORAGE_FILE)?;
    Ok(())
}
