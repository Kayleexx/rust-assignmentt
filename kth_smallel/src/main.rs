// function that returns the kth smallest element in a given array.

use std::io;

fn kth_small(arr: &mut Vec<i32>, k: usize) -> Option<i32> {
    arr.sort();

    if k > 0 && k <= arr.len() {
        Some(arr[k - 1])
    } else {
        None
    }
}

fn main() {
    println!("Enter the array of integers: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Invalid input"))
        .collect();

    println!("Enter the value of k: ");

    let mut k_input = String::new();
    io::stdin()
        .read_line(&mut k_input)
        .expect("Failed to read input");

    let k: usize = k_input.trim().parse().expect("Invalid input");

    match kth_small(&mut arr.clone(), k) {
        Some(smallest) => println!("{}th smallest element is {}", k, smallest),
        None => println!("Invalid value of k"),
    }
}
