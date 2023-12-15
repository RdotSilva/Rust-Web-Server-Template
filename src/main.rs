use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use reqwest::Client as HttpClient;

use async_trait::async_trait;

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn get(&mut self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    fn get_all(&mut self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn delete(&mut self, id: &u64) -> Option<&Task> {
        self.tasks.remove(id)
    }

    fn update(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    // User data related functions
    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u: &&User| u.username == username)
    }

    // Database saving functions
    fn save_to_file(&self) -> std::io::Result<()> {
        // Convert hashmap to JSON string
        let data: String = serde_json::to_string(&self)?;

        // Create the database JSON file
        let mut file: fs::File = fs::File::create("database.json")?;

        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
