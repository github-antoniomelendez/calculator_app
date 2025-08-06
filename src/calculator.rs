use std::io;
 
 // This defines the struct to represent the normal calculator, this is where we use struct
pub struct Calculator;

// This is an implementation block, defines the calculator methods, this is where we use impl
impl Calculator {
    // This is a public function and performs the logic of the basic calculator functions, this function uses ownership
    pub fn calculate(&self) {
        let mut input = String::new(); // This string stores user input

        // This gets the first number
        println!("Enter first number: ");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let a: f64 = match input.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Invalid number.");
                return;
            }
        };

        // This is used for getting the operator
        input.clear();
        println!("Enter operator (+, -, *, /): ");
        io::stdin().read_line(&mut input).expect("Failed to read input"); // In case the user's input is not correct
        let op = input.trim().to_string();

        // This is used to get the second number, just like when getting the first number
        input.clear();
        println!("Enter second number: ");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let b: f64 = match input.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Invalid number.");
                return;
            }
        };

        // This matches the operator and gets the result, this uses conditionals in the match and if
        let result = match op.as_str() {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => {
                if b == 0.0 {
                    println!("Cannot divide by zero!");
                    0.0
                } else {
                    a / b
                }
            }
            _ => {
                println!("Unsupported operator");
                0.0
            }
        };

        println!("Result: {}", result); // prints result
    }  // Ends public function
} // Ends implementation block
