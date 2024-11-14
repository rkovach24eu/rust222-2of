use std::io::{self, BufRead};

fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut count = 0;
    
    // Iterate through the array to check each contiguous subarray of length m
    for i in 0..=s.len() - m as usize {
        let sum: i32 = s[i..i + m as usize].iter().sum();
        if sum == d {
            count += 1;
        }
    }
    
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    // Read the array of integers (the chocolate squares)
    let s: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();
    
    // Read d and m (birth day and birth month)
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();
    
    let d = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();
    
    // Call the birthday func
