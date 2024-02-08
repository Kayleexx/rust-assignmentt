//function that returns the index of the first occurrence of a given number.


use std::io;

fn first_occ(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x|x == target)
}

fn main() {
    println!("Enter the array: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");

    let numbers: Vec<i32> = input.trim().split_whitespace().map(|x|x.parse().expect("Invalid input")).collect();

    println!("Enter the number to search for");
    let mut target = String::new();
    io::stdin().read_line(&mut target)
    .expect("Failed to read line");

    let target:i32 = target.trim().parse().expect("Invalid input");

    if let Some(index) = first_occ(&numbers, target) {

        println!("The index of {} is found in {}", target, index);

    } else {
        println!("The number is not found");
    }
}

