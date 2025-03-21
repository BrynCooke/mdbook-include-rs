use std::fmt;

fn function_with_display_markers() {
    // Setup code that shouldn't be displayed
    let x = 10;
    let y = 20;
    
    // DISPLAY START
    // This part should be visible
    let result = x + y;
    println!("Result: {}", result);
    // DISPLAY END
    
    // Cleanup code that shouldn't be displayed
    println!("Done!");
}

fn function_with_display_start_only() {
    // Hidden setup
    let name = "World";
    
    // DISPLAY START
    // Everything from here should be visible
    println!("Hello, {}!", name);
    let message = format!("Welcome to Rust");
    println!("{}", message);
}

fn function_with_display_end_only() {
    // Everything until the end marker should be visible
    println!("This is visible");
    let calculation = 42;
    println!("The answer is: {}", calculation);
    
    // DISPLAY END
    // This is hidden
    println!("Cleanup operations");
}

fn function_without_markers() {
    // This function has no display markers
    // The entire body should be visible by default
    let a = 5;
    let b = 7;
    let sum = a + b;
    println!("{} + {} = {}", a, b, sum);
}