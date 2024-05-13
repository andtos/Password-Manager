use std::{fs, process};

use password_manager::{
    logged_in::{logged_in_user_loop, prompt_input},
    login::{Login, User},
    storage::{STORAGE_DIRECTORY, STORAGE_FILE},
};

fn main() {
    println!("Welcome to password manager...");
    if fs::metadata(STORAGE_FILE).is_ok() {
        unknown_user_loop();
    } else {
        match fs::create_dir_all(STORAGE_DIRECTORY) {
            Ok(_) => match fs::File::create(STORAGE_FILE) {
                Ok(_) => unknown_user_loop(),
                Err(_) => println!("Storage file could not be created... Shutting down"),
            },
            _ => println!("Error creating storage directory... Shutting down"),
        }
    }
}

fn login(user: Option<User>) {
    match user {
        None => {
            println!("Incorrect Username or Pasword");
            unknown_user_loop();
        }
        Some(user) => {
            println!("Welcome back {}", user.username);
            logged_in_user_loop(user);
        }
    }
}

fn register(user: Option<User>) {
    match user {
        None => {
            unknown_user_loop();
        }
        Some(user) => {
            println!("Welcome {}", user.username);
            logged_in_user_loop(user);
        }
    }
}

fn unknown_user_loop() {
    loop {
        let binding = prompt_input("Enter a command");
        let user_res = binding.as_str();
        match user_res.to_lowercase().as_str() {
            "quit" => {
                println!("Shutting down");
                process::exit(0);
            }
            "login" => {
                let username = prompt_input("Enter Username");
                let password = prompt_input("Enter Password");
                let user = Login::login(&username, &password);
                break login(user);
            }
            "register" => {
                let username = prompt_input("Enter New Username");
                let password = prompt_input("Enter New Password");
                let user = Login::register(&username, &password);
                break register(user);
            }
            _ => {
                println!("Available commands: help, quit, login, register");
            }
        };
    }
}
