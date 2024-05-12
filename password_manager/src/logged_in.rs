use std::{io, process};

use clipboard::{ClipboardContext, ClipboardProvider};

use crate::{
    login::User, password::generate_secure_password, storage::{add_site, retrieve_password}
};

pub fn logged_in_user_loop(user: User) {
    loop {
        let binding = prompt_input("Generate (new) password or Retrieve (old) password");
        let user_res = binding.as_str();
        match user_res.to_lowercase().as_str() {
            "quit" => {
                println!("Shutting down");
                process::exit(0);
            }
            "new" => {
                let site_name = prompt_input("Enter nickname for site");
                let username = prompt_input("Enter the new username");
                let secure_password = generate_secure_password();
                match add_site(&user.username, &username, site_name, &secure_password) {
                    Ok(_) => {
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        ctx.set_contents(secure_password.to_string()).unwrap();
                        println!("A secure password has been generated and copied to clipboard");
                        println!("Information for the site has been saved successfuly");
                        println!("You can retrieve it later using the site nickname");
                    }
                    Err(e) => {
                        println!("Something went wrong: {}", e.0);
                    }
                }
            }
            "old" => {
                let site_name = prompt_input("Enter site nickname");
                match retrieve_password(&user.username, site_name) {
                    Some((username, password)) => {
                        println!("username: {}", username);
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        ctx.set_contents(password).unwrap();
                        println!("password copied to clipboard");
                    }
                    None => {
                        println!("Could not retrieve information for that site")
                    }
                }
            }
            _ => {
                println!("Available commands: new, old, quit");
            }
        };
    }
}

pub fn prompt_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_lowercase().to_string()
}
