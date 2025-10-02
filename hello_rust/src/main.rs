fn main() {
    

        
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


