use std::io;

fn is_pal(s: &str) -> bool {
    let reverse = s.chars().rev().collect::<String>(); //iterates over the characters of a string, reverses their order, and collects them into a new string
    s == reverse
}

fn main() {
    println!("Enter a string: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");

    let input = input.trim(); //Remove any empty space or newline character at the end of a string.

    if is_pal(&input) {
        println!("{} is a palindrome", input);
        
    } else {
        println!("{} is not a palindrome", input);
    }
}