
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct Joke {
    id: u32,
    category: String,
    content: String,
}

fn get_jokes() -> HashMap<u32, Joke> {
    let mut jokes = HashMap::new();
    jokes.insert(1, Joke {
        id: 1,
        category: "Programming".to_string(),
        content: "Why do programmers prefer dark mode? Because light attracts bugs!".to_string(),
    });
    jokes.insert(2, Joke {
        id: 2,
        category: "Programming".to_string(),
        content: "A SQL query walks into a bar, walks up to two tables and asks: 'Can I join you?'".to_string(),
    });
    jokes.insert(3, Joke {
        id: 3,
        category: "General".to_string(),
        content: "Why did the scarecrow win an award? He was outstanding in his field!".to_string(),
    });
    jokes
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("🎭 Welcome to the Rust Joke API! Try /jokes or /joke/1")
}

async fn get_all_jokes() -> impl Responder {
    let jokes = get_jokes();
    let jokes_vec: Vec<&Joke> = jokes.values().collect();
    HttpResponse::Ok().json(jokes_vec)
}

async fn get_joke_by_id(path: web::Path<u32>) -> impl Responder {
    let joke_id = path.into_inner();
    let jokes = get_jokes();
    
    match jokes.get(&joke_id) {
        Some(joke) => HttpResponse::Ok().json(joke),
        None => HttpResponse::NotFound().body("Joke not found! Try IDs 1-3")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting Rust Joke API server at http://localhost:8080");
    println!("📚 Available endpoints:");
    println!("   GET /                 - Welcome message");
    println!("   GET /jokes            - Get all jokes");
    println!("   GET /joke/{{id}}       - Get joke by ID (1-3)");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/jokes", web::get().to(get_all_jokes))
            .route("/joke/{id}", web::get().to(get_joke_by_id))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}