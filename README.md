# 🦀 Rust Learning Projects

A collection of beginner-friendly Rust projects to learn the fundamentals of Rust programming.

---

## 📚 Table of Contents

1. [Hello Rust](#-1-hello-rust)
2. [Joke API](#-2-themed-hello-world---joke-api)

---

## 👋 1. Hello Rust

A classic "Hello, World!" program to get started with Rust.

### 🎯 Objective

Create a simple program that prints "Hello, World!" to demonstrate basic Rust syntax and project setup.

### 📁 Project Structure

```
hello_rust/
├── Cargo.toml
└── src/
    └── main.rs
```

### 🚀 How to Run

```bash
cd hello_rust
cargo run
```

### 📤 Expected Output

```
Hello, World!
```

### ✨ Features Demonstrated

- Basic Rust project structure
- Using `cargo` to build and run projects
- The `println!` macro for console output
- Entry point with `fn main()`

---

## 🎭 2. Themed Hello World - Joke API

A simple RESTful API that serves programming jokes, demonstrating web server capabilities in Rust.

### 🎯 Objective

Create a simple RESTful API that serves programming jokes, demonstrating web server capabilities in Rust.

### 🛠️ Technology Used

- **Actix-web framework** - For web server functionality
- **Serde** - For JSON serialization

### 📁 Project Structure

```
joke_api/
├── Cargo.toml
└── src/
    └── main.rs
```

### 📦 Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
actix-web = "4.4"
serde = { version = "1.0", features = ["derive"] }
```

### ✨ Features Demonstrated

- Web server creation with Actix-web
- JSON serialization with Serde
- Multiple API endpoints
- HTTP route handling
- Basic error handling

### 🚀 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Welcome message |
| `GET` | `/jokes` | Get all jokes |
| `GET` | `/joke/{id}` | Get specific joke by ID (1-3) |

### 🏃 How to Run

```bash
cd joke_api
cargo run
```

Then visit http://localhost:8080 in your browser

### 🧪 Testing the API

Try these endpoints in your browser or with curl:

- **http://localhost:8080/** - Welcome message
- **http://localhost:8080/jokes** - All jokes in JSON format
- **http://localhost:8080/joke/1** - Specific joke by ID



