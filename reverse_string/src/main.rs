//reverse a string 

use std::io;

fn reverse_str(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    println!("Enter a string:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    
    let trimmed_input = input.trim(); // Remove trailing newline character
    
    let reversed = reverse_str(trimmed_input);
    println!("Reversed string: {}", reversed);
}
