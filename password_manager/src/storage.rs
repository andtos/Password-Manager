use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use serde::{Deserialize, Serialize};
pub const STORAGE_FILE: &str = "storage/storage.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Storage {
    data: HashMap<String, String>,
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
}

pub fn add_to_storage(username: String, password: String) -> Result<(), std::io::Error> {
    let mut file = File::open(STORAGE_FILE)?;
    let mut deserialized: Storage = serde_json::from_reader(&file)?;
    deserialized.data.insert(username, password);
    let json_string = serde_json::to_string_pretty(&deserialized).unwrap();
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

pub fn create_new_storage() -> Result<fs::File, std::io::Error> {
    let file = fs::File::create(STORAGE_FILE)?;
    Ok(file)
}
