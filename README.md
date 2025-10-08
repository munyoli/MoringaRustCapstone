# Getting Started with Rust - A Beginner's Guide to Systems Programming

**Capstone Project by MUNYOLI MWENDE**

---

## 📋 Table of Contents

1. [Overview](#-overview)
2. [System Requirements](#-system-requirements)
3. [Installation & Setup](#-installation--setup)
4. [Project 1: Hello Rust](#-project-1-hello-rust)
5. [Project 2: Joke API](#-project-2-joke-api)
6. [AI Prompt Journal](#-ai-prompt-journal)
7. [Common Issues & Fixes](#-common-issues--fixes)
8. [References](#-references)

---

## 🎯 Overview

### Why Rust?

- Rust is gaining popularity for systems programming, web assembly, and embedded systems
- It offers memory safety without garbage collection
- Growing demand for Rust developers in the industry
- Unique ownership model that's different from other languages

### What is Rust?

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It achieves memory safety without garbage collection through its innovative ownership system.

### Where is it used?

- Web browsers (Firefox uses Rust in parts)
- Operating systems
- Game engines
- Web Assembly applications
- Blockchain and cryptocurrency projects
- Embedded systems

**Real-world Example:** Firefox's CSS engine (Stylo) is written in Rust, providing significant performance improvements while maintaining memory safety.

---

## 💻 System Requirements

**OS:** Linux, macOS, or Windows

**Tools Required:**
- Rustup (Rust toolchain installer)
- VS Code or any text editor
- Rust analyzer extension (recommended)

**Note:** No additional packages required - Rust comes with Cargo, its built-in package manager.

---

## 🔧 Installation & Setup

### Step 1: Install Rust

**On macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**On Windows:**
Download and run `rustup-init.exe` from https://rustup.rs/

### Step 2: Verify Installation

```bash
rustc --version
cargo --version
```

### Step 3: Configure Your Editor

Install Rust analyzer extension in VS Code for better development experience.

---

## 👋 Project 1: Hello Rust

### Objective

Create a simple "Hello World" CLI application that demonstrates basic Rust concepts.

### Project Structure

```
hello_rust/
├── Cargo.toml
└── src/
    └── main.rs
```

### Configuration (Cargo.toml)

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### Code Example (src/main.rs)

```rust
// This is the main function - entry point of the program
fn main() {
    // Print to console
    println!("Hello, Rust world!");
    
    // Variables and immutability
    let name = "Rust Learner";
    println!("Welcome, {}!", name);
    
    // Mutable variable
    let mut counter = 0;
    counter += 1;
    println!("Counter: {}", counter);
    
    // Basic function call
    let result = add_numbers(5, 3);
    println!("5 + 3 = {}", result);
    
    // Control flow
    if result > 5 {
        println!("The result is greater than 5!");
    } else {
        println!("The result is 5 or less");
    }
}

// Function definition with return type
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b // No semicolon means this is the return value
}
```

### How to Run

```bash
# Navigate to project directory
cd hello_rust

# Build and run
cargo run
```

### Expected Output

```
Hello, Rust world!
Welcome, Rust Learner!
Counter: 1
5 + 3 = 8
The result is greater than 5!
```

### Key Concepts Demonstrated

- Basic Rust project structure
- Using `cargo` to build and run projects
- The `println!` macro for console output
- Variables and mutability
- Functions with return types
- Control flow (if/else)

---

## 🎭 Project 2: Joke API (Themed Hello World)

### Objective

Create a simple RESTful API that serves programming jokes, demonstrating web server capabilities in Rust.

### Technology Used

- **Actix-web framework** - For web server functionality
- **Serde** - For JSON serialization

### Project Structure

```
joke_api/
├── Cargo.toml
└── src/
    └── main.rs
```

### Dependencies (Cargo.toml)

```toml
[dependencies]
actix-web = "4.4"
serde = { version = "1.0", features = ["derive"] }
```

### Features Demonstrated

- Web server creation with Actix-web
- JSON serialization with Serde
- Multiple API endpoints
- HTTP route handling
- Basic error handling

### API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Welcome message |
| `GET` | `/jokes` | Get all jokes |
| `GET` | `/joke/{id}` | Get specific joke by ID (1-3) |

### How to Run

```bash
cd joke_api
cargo run
```

Then visit http://localhost:8080 in your browser

### Testing the API

- **http://localhost:8080/** - Welcome message
- **http://localhost:8080/jokes** - All jokes in JSON format
- **http://localhost:8080/joke/1** - Specific joke by ID

---

## 📝 AI Prompt Journal

### Prompt 1: Understanding Rust Project Structure

**Prompt:**
```
I'm a student working on my capstone project and I'm new to Rust. I've started setting up 
my development environment but need help understanding Rust project structure and the 
technology stack.

Here's my current understanding of Rust projects:
• It seems to be a systems programming language that focuses on [safety, performance, 
  concurrency]
• It appears to use [Cargo as the build tool and package manager]
• The folder structure seems to follow [a standard Cargo project layout with src/ and 
  target/ directories]

My project structure:
my_rust_project/
├── src/
│   └── main.rs
├── target/
├── Cargo.toml
└── Cargo.lock

Could you:
1. Validate my understanding of Rust's project structure
2. Explain what each folder contains and its purpose
3. Help me understand the difference between Cargo.toml and Cargo.lock
4. Point out where I should write my main application code
5. Suggest 3-5 beginner-friendly Rust concepts I should focus on first
```

**AI Response Summary:** The AI provided detailed installation instructions using rustup, explained how to verify installation with `rustc --version`, and showed how to create a basic Rust project using Cargo.

**Helpfulness:** 9/10 - Comprehensive and accurate, but didn't mention the Rust analyzer extension initially.

---

### Prompt 2: Code Organization and Modules

**Prompt:**
```
I need to work on implementing error handling / adding a specific module / fixing the 
linker configuration in my Rust codebase, but I'm not sure where the code for this feature 
should live or how it's currently organized.

My approach so far:
• I've searched for keywords like [main, mod, use, pub]
• I looked in [src/ directory, Cargo.toml] which seemed relevant
• I think the feature might relate to [the main application entry point / module system / 
  build configuration]

Can you help me:
1. Evaluate my search approach and suggest improvements
2. Identify which files should contain [my specific feature]
3. Suggest specific search terms or Rust-specific patterns (like impl, trait, struct)
4. Explain how to properly organize code in Rust (modules, crates, files)
5. Recommend a step-by-step investigation process

Also, what questions could I ask myself as I'm exploring the code?
• How do I know if something should be in main.rs vs a separate module?
• What patterns should I look for in well-organized Rust projects?
```

**AI Response Summary:** Confirmed Rust is a systems programming language focused on memory safety, performance, and concurrency. Validated that Cargo is the official build tool and package manager and corrected identification of standard project structure.

**Helpfulness:** 8/10 - Good conceptual explanation but needed simpler, more practical examples.

---

### Prompt 3: Compilation Errors & Troubleshooting

**Prompt:**
```
I'd like you to act as a senior Rust developer who deeply understands common compilation 
errors and the Rust toolchain. I'm a student working on my capstone project and trying to 
make sense of compilation failures.

Here's the error I encountered:
Compiling hello_rust v0.1.0 (C:\Users\USER PC\CODING\CAPSTONE PROJECT\hello_rust)
error: linker `link.exe` not found
  
 = note: program not found
note: the msvc targets depend on the msvc linker but `link.exe` was not found
note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio 
      were installed with the Visual C++ option.

My development environment:
• Operating System: Windows
• Rust version: rustc 1.90.0
• Installed via: rustup
• Editor: VS Code
• Toolchain: stable-msvc (default)

Based on this error, my current understanding is:
• The system seems to be looking for a linker tool called link.exe
• I think Rust and Visual Studio are related
• The error appears to represent a missing dependency in my development environment

Could you, as a senior Rust developer:
1. Validate my understanding of what a linker is and why Rust needs it
2. Help me recognize core concepts: compiler vs linker, toolchains (MSVC vs GNU)
3. Explain the relationship between Rust, the operating system, and required tools
4. Clarify Rust-specific terminology (rustup, cargo, rustc, toolchain targets)
5. Identify other common compilation errors I should be aware of
```

**AI Response Summary:** The AI listed common errors like "borrow after move", "cannot borrow as mutable", and "expected type" errors with explanations and fixes.

**Helpfulness:** 9/10 - Very practical and helped anticipate issues before they happened.

---

## 🔧 Common Issues & Fixes

### Issue 1: Linker Not Found Error (MSVC Toolchain Dependency)

**Problem:** "Linker Not Found Error (MSVC Toolchain Dependency)"

One common compilation error encountered when setting up Rust on Windows systems is the missing linker error. This occurs because Rust's default MSVC toolchain requires `link.exe` from Visual Studio Build Tools, which is not included in standard Windows installations.

**Solution:**
Install Visual Studio Build Tools 2022 with the C++ development workload to provide the required MSVC linker. Download from the official Visual Studio downloads page, select 'Desktop development with C++' during installation, and ensure MSVC v143 build tools and Windows SDK components are included. After installation, restart the terminal to update environment variables.

---

### Issue 2: "Command not found: cargo"

**Problem:** Cargo isn't in PATH after installation

**Solution:**
```bash
source $HOME/.cargo/env
# or restart your terminal
```

---

### Issue 3: Ownership/Borrowing Errors

**Problem:** Common compiler errors about moving values

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1); // Error: value borrowed after move
```

**Solution:**
```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // Use clone for deep copy
println!("{}", s1); // Now works
```

---

### Issue 4: Missing Semicolon in Return Values

**Problem:** Function returns `()` instead of expected type

**Solution:** Remove semicolon from the last expression in functions

---

## 📚 References

### Official Documentation
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Standard Library Documentation](https://doc.rust-lang.org/std/)

### Tutorials & Learning Resources
- [Rustlings - Small exercises](https://github.com/rust-lang/rustlings)
- [Exercism Rust Track](https://exercism.org/tracks/rust)
- [Comprehensive Rust 🦀](https://google.github.io/comprehensive-rust/)

### Community
- [Rust Programming Language Forum](https://users.rust-lang.org/)
- [Rust Subreddit](https://www.reddit.com/r/rust/)
- [Rust Discord Community](https://discord.gg/rust-lang)

### Tools
- [Rust Playground](https://play.rust-lang.org/)
- [Rust Analyzer](https://rust-analyzer.github.io/)

---

