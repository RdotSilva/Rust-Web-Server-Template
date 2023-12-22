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

    /// Insert an item into the database
    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    /// Get an item from the database
    fn get(&mut self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    /// Get all items from the database
    fn get_all(&mut self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    /// Delete an item from the database
    fn delete(&mut self, id: &u64) -> Option<Task> {
        self.tasks.remove(id)
    }

    /// Update an item in the database
    fn update(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    /// User data related functions

    /// Insert a user into the database
    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    /// Get a user by name from the database
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

    fn load_from_file() -> std::io::Result<Self> {
        let file_content: String = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>,
}

// Insert the task into database and save to file
async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

// Read a task from the database
async fn read_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();

    match db.get(&id.into_inner()) {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}

// Read all tasks from database
async fn read_all_tasks(app_state: web::Data<AppState>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();

    let tasks: Vec<&Task> = db.get_all();
    HttpResponse::Ok().json(tasks)
}

// Update a task in the database
async fn update_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();

    db.insert(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Database = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    let data: web::Data<AppState> = web::Data::new(AppState { db: Mutex::new(db) });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            .route("/task", web::post().to(create_task))
            .route("/task", web::get().to(read_all_tasks))
            .route("/task/{id}", web::get().to(read_task))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
