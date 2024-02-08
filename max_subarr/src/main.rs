//max. subarray 

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the array: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Parse error"))
        .collect();
    let max_sum = max_subarray_sum(&arr); 
    println!("Maximum subarray sum: {}", max_sum);
}

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for &num in arr.iter().skip(1) {
        current_sum = current_sum.max(num);
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}
