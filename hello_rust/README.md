```markdown
# Getting Started with Rust - A Beginner's Guide

**Capstone Project by Munyoli Mwende**

A comprehensive guide to setting up and understanding Rust programming language fundamentals, designed for beginners entering the world of systems programming.

---

## 📋 Table of Contents

- [Title & Objective](#title--objective)
- [Quick Summary](#quick-summary-of-the-technology)
- [System Requirements](#system-requirements)
- [Installation & Setup](#installation--setup-instructions)
- [Minimal Working Example](#minimal-working-example)
- [AI Prompt Journal](#ai-prompt-journal)
- [Common Issues & Fixes](#common-issues--fixes)
- [References](#references)

---

## 🎯 Title & Objective

**Title:** Getting Started with Rust - A Beginner's Guide to Systems Programming

**Technology Chosen:** Rust Programming Language

**Why I Chose It:**
- Rust is gaining popularity for systems programming, web assembly, and embedded systems
- It offers memory safety without garbage collection
- Growing demand for Rust developers in the industry
- Unique ownership model that's different from other languages

**End Goal:** Create a simple "Hello World" CLI application that also demonstrates basic Rust concepts.

---

## 📖 Quick Summary of the Technology

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

## 🚀 Installation & Setup Instructions

### Step 1: Install Rust

```bash
# On macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# On Windows, download and run rustup-init.exe from https://rustup.rs/
```

### Step 2: Verify Installation

```bash
rustc --version
cargo --version
```

### Step 3: Configure Your Editor

Install Rust analyzer extension in VS Code for better development experience.

---

## 📝 Minimal Working Example

### Project Structure

```text
hello_rust/
├── Cargo.toml
└── src/
    └── main.rs
```

### Code (src/main.rs)

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
    a + b  // No semicolon means this is the return value
}
```

### Cargo.toml (Project Configuration)

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### How to Run

```bash
# Navigate to project directory
cd hello_rust

# Build and run
cargo run
```

### Expected Output

```text
Hello, Rust world!
Welcome, Rust Learner!
Counter: 1
5 + 3 = 8
The result is greater than 5!
```

---

## 🤖 AI Prompt Journal

### Prompt 1: Understanding Rust Project Structure

**Prompt via Claude:**

I'm a student working on my capstone project and I'm new to Rust. I've started setting up my development environment but need help understanding Rust project structure and the technology stack.

Here's my current understanding of Rust projects:
- It seems to be a systems programming language that focuses on [safety, performance, concurrency]
- It appears to use [Cargo as the build tool and package manager]
- The folder structure seems to follow [a standard Cargo project layout with src/ and target/ directories]

**My project structure:**
```
my_rust_project/
├── src/
│   └── main.rs
├── target/
├── Cargo.toml
└── Cargo.lock
```

**Key configuration files:**
```toml
[From my Cargo.toml]
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
# (list any dependencies I've added)
```

**Could you:**
1. Validate my understanding of Rust's project structure and correct any misconceptions
2. Explain what each folder (src/, target/, etc.) contains and its purpose
3. Help me understand the difference between Cargo.toml and Cargo.lock
4. Point out where I should write my main application code and how to organize modules
5. Explain common development environment issues on Windows (like the linker error I encountered)
6. Suggest 3-5 beginner-friendly Rust concepts I should focus on first for my capstone project

**I'm particularly confused about:**
- [The MSVC linker requirement and toolchain setup]
- [How Cargo manages dependencies and builds]
- [Where to place different types of files in my project]

**After your explanation, could you suggest:**
- A small coding exercise to verify my understanding of Rust basics
- Best practices for organizing a capstone-level Rust project
- Resources or documentation I should bookmark for reference

**AI Response Summary:** The AI provided detailed installation instructions using rustup, explained how to verify installation with rustc --version, and showed how to create a basic Rust project using Cargo.

**Helpfulness:** 9/10 - Comprehensive and accurate, but didn't mention the Rust analyzer extension initially.

---

### Prompt 2: Module Organization and Code Structure

**Prompt:**

I need to work on implementing error handling / adding a specific module / fixing the linker configuration in my Rust codebase, but I'm not sure where the code for this feature should live or how it's currently organized.

**My approach so far:**
- I've searched for keywords like [main, mod, use, pub]
- I looked in [src/ directory, Cargo.toml] which seemed relevant
- I think the feature might relate to [the main application entry point / module system / build configuration]

**My project structure:**
```
my_rust_project/
├── src/
│   ├── main.rs
│   ├── lib.rs (if applicable)
│   └── [any other .rs files]
├── target/
│   ├── debug/
│   └── release/
├── Cargo.toml
├── Cargo.lock
└── .cargo/
    └── config.toml (if exists)
```

**Based on my search, these files might be relevant, but I'm not sure:**
- src/main.rs - Contains the main function but not sure how to organize additional features
- Cargo.toml - Has dependencies but unclear how to configure build settings
- .cargo/config.toml - Don't know if I need this file or what it does

**Can you help me:**
1. Evaluate my search approach and suggest improvements for finding Rust feature implementations
2. Identify which files and directories most likely should contain [my specific feature]
3. Suggest specific search terms or Rust-specific patterns that would be more effective (like impl, trait, struct)
4. Explain how to properly organize code in Rust (modules, crates, files)
5. Recommend a step-by-step investigation process to understand where different parts of my feature should be implemented

**Also, what questions could I ask myself as I'm exploring the code to ensure I'm on the right track?**
- How do I know if something should be in main.rs vs a separate module?
- What patterns should I look for in well-organized Rust projects?

**After your guidance, could you give me a small challenge to test my understanding of how to navigate and organize Rust code?**

**AI Response Summary:** Confirmed Rust is a systems programming language focused on memory safety, performance, and concurrency. Validated that Cargo is the official build tool and package manager and corrected identification of standard project structure.

**Helpfulness:** 8/10 - Good conceptual explanation but needed simpler, more practical examples.

---

### Prompt 3: Compilation Errors & Troubleshooting

**Prompt: Rust Compilation Errors & Troubleshooting Prompt - Capstone Project**

I'd like you to act as a senior Rust developer who deeply understands common compilation errors and the Rust toolchain. I'm a student working on my capstone project and trying to make sense of compilation failures and error messages I'm encountering.

**Here's the error I encountered:**
```
Compiling hello_rust v0.1.0 (C:\Users\USER PC\CODING\CAPSTONE PROJECT\hello_rust)
error: linker `link.exe` not found
  |
  = note: program not found
note: the msvc targets depend on the msvc linker but `link.exe` was not found
note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio 
      were installed with the Visual C++ option.
note: VS Code is a different product, and is not sufficient.
error: could not compile `hello_rust` due to previous error
```

**My development environment:**
- Operating System: Windows [version]
- Rust version: rustc 1.90.0
- Installed via: rustup
- Editor: VS Code
- Toolchain: stable-msvc (default)

**Based on this error, my current understanding is:**
- The system seems to be looking for a linker tool called link.exe that compiles my Rust code
- I think Rust and Visual Studio are related because the error mentions Visual Studio Build Tools
- The error message appears to represent a missing dependency in my development environment
- I'm confused about why a programming language needs Visual Studio components when I'm just writing code in a text editor

**Could you, as a senior Rust developer:**
1. Validate my current understanding of what a linker is and why Rust needs it
2. Help me recognize the core concepts: compiler vs linker, toolchains (MSVC vs GNU), and build process
3. Explain the relationship between Rust, the operating system, and required development tools in practical terms
4. Clarify any Rust-specific terminology I might be missing (rustup, cargo, rustc, toolchain targets)
5. Help me connect these compilation concepts to the actual process of building my capstone project
6. Identify other common compilation errors I should be aware of as a Rust beginner

**AI Response Summary:** The AI listed common errors like "borrow after move", "cannot borrow as mutable", and "expected type" errors with explanations and fixes.

**Helpfulness:** 9/10 - Very practical and helped anticipate issues before they happened.

---

## 🔧 Common Issues & Fixes

### Issue 1: Rustup Installation Problems

**Problem:** "Linker Not Found Error (MSVC Toolchain Dependency)"

One common compilation error encountered when setting up Rust on Windows systems is the missing linker error. This occurs because Rust's default MSVC toolchain requires link.exe from Visual Studio Build Tools, which is not included in standard Windows installations.

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

**Problem:** Function returns () instead of expected type

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

