use std::io;

fn find_median(arr: &[i32]) -> f64 {
    
    let len = arr.len();

    if len % 2 == 0 {

        let mid_right = len/2;
        let mid_left = mid_right -1;
        return (arr[mid_left] as f64 + arr[mid_right] as f64) / 2.0;
    } else {
        let mid = len /2;
        return arr[mid] as f64;
    }
 
}

fn main() {
    println!("Enter the sorted array of integers: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");

    let arr: Vec<i32> = input.trim().split_whitespace()
    .map(|x| x.parse().expect("Invalid input"))
    .collect();

    let median = find_median(&arr);
    println!("Median: {}", median);
}