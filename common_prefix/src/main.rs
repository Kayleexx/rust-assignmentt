//function that finds the longest common prefix of a given set of strings.

use std::io;

fn common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    for (index, character) in strings[0].chars().enumerate() {
        if strings.iter().all(|s| s.chars().nth(index) == Some(character)) {
            continue;
        } else {
            return strings[0][..index].to_string();
        }
    }

    strings[0].clone()
}

fn main() {
    println!("Enter a set of strings:");

    let mut input = String::new();
    if let Err(error) = io::stdin().read_line(&mut input) {
        eprintln!("Error reading input: {}", error);
        return;
    }

    let strings: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();

    let longest_prefix = common_prefix(&strings);

    println!("Longest common prefix: {}", longest_prefix);
}
