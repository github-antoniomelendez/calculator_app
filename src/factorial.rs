use std::io;
// Defines the struct to represent the factorial calculator, this is where we use struct
pub struct Factorial;
// begins implementation block, this is where we use impl
impl Factorial {
    // This public function is recursive, it gets the factorial of a number
    pub fn compute(&self, n: u32) -> u32 {
        if n == 0 {
            1
        } else {
            n * self.compute(n - 1) // This is the recursive call
        }
    }

    // This is a public function, it interacts with the user
    pub fn run(&self) { // This public function uses ownership
        let mut input = String::new();

        // This loop continues until the user input is not a negative number
        loop {
            input.clear(); // Clears input
            println!("Enter a non-negative integer: ");
            io::stdin().read_line(&mut input).expect("Failed to read input"); 

            // Matches the user input with the correct result
            match input.trim().parse::<u32>() {
                Ok(n) => {
                    let result = self.compute(n);
                    println!("Factorial of {} is {}", n, result); // This prints the resul, the factorial of the number
                    break;
                }
                Err(_) => {
                    println!("Invalid input. Please enter a non-negative integer."); // This shows the user the input is incorrect
                }
            }
        }
    }
} // ends factorial implementation block
