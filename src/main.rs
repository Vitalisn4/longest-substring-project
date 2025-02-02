use std::io;
use longest_substring::longest_unique_substring;

fn main() {
    loop {
        // Prompt the user for input
        println!("Enter a string to find the longest substring without repeating characters:");

        // Create a mutable String to store the input
        let mut input = String::new();

        // Read input from the user
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Remove the newline character at the end of input
        let input = input.trim();

        // Call the function from lib.rs and print the result
        let result = longest_unique_substring(input);
        println!("The longest substring without repeating characters is: {}", result);

        // Ask the user if they want to try another example
        println!("\nWould you like to try another example? (yes/no):");

        // Create a mutable String to store the user's answer
        let mut try_again = String::new();
        io::stdin()
            .read_line(&mut try_again)
            .expect("Failed to read line");

        // Remove newline and extra spaces from the user's answer
        let try_again = try_again.trim().to_lowercase();

        // If the user says "no", break the loop and exit
        if try_again == "no" {
            println!("Exiting program. Goodbye!");
            break;
        }
    }
}
