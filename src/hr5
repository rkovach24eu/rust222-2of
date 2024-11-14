use std::io::{self, BufRead};

fn plus_minus(arr: &[i32]) {
    let total = arr.len() as f64;
    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut zero_count = 0;

    // Count the positive, negative, and zero elements
    for &num in arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    // Calculate the ratios
    let positive_ratio = positive_count as f64 / total;
    let negative_ratio = negative_count as f64 / total;
    let zero_ratio = zero_count as f64 / total;

    // Print the results with six decimal places
    println!("{:.6}", positive_ratio);
    println!("{:.6}", negative_ratio);
    println!("{:.6}", zero_ratio);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read the number of elements (though we don't need to use it directly in this solution)
    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Parse the array from the second line of input
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Call the plus_minus function with the input array
    plus_minus(&arr);
}
