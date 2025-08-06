// In Rust this is how we declare modules, main runs module for calculator and for factorial
mod calculator;
mod factorial;

// This brings the structs into scope so we can use them
use calculator::Calculator;
use factorial::Factorial;
use std::io;

// This is the main function, it runs the program
fn main() {
    // Let creates instances of our struct objects, these are immutable variables, they cannot be reassigned or changed
    let calc = Calculator;
    let fact = Factorial;
    let mut input = String::new(); // This string stores user input, it is also reusable, it is a mutable variable because it can change

    // This loop displays the menu, it runs until the user decides to quit
    loop {
        println!("\n=== Menu ===");
        println!("1. Basic Calculator");
        println!("2. Factorial");
        println!("3. Quit");
        println!("Enter your choice: ");

        input.clear(); // This clears the input and reads a new line
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // This matches the user input according to the user choices
        match input.trim() {
            "1" => calc.calculate(), // This calls the normal calculator module
            "2" => fact.run(), // This calls the factorial calculator module
            "3" => {
                println!("Goodbye!");
                break; // This exits the loop and ends the program
            }
            _ => println!("Invalid choice. Please enter 1, 2, or 3."), // This handles incorrect user input
        }
    }
}
