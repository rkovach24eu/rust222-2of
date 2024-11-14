use std::io;

fn mini_max_sum(arr: Vec<i64>) {
    // Calculate the total sum of the array
    let total_sum: i64 = arr.iter().sum();

    // Find the minimum and maximum elements in the array
    let min_element = arr.iter().min().unwrap();
    let max_element = arr.iter().max().unwrap();

    // Minimum sum is total_sum minus the max element
    let min_sum = total_sum - max_element;

    // Maximum sum is total_sum minus the min element
    let max_sum = total_sum - min_element;

    // Print the results
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    // Read the input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into a vector of integers
    let arr: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer"))
        .collect();

    // Call the function
    mini_max_sum(arr);
}
