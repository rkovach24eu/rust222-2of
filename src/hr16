use std::io::{self, BufRead};
use std::fs::File;
use std::env;
use std::io::Write;

fn divisibleSumPairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut mod_count = vec![0; k as usize];  // Store frequencies of remainders (mod k)
    let mut pair_count = 0;

    // Traverse the array
    for &num in ar {
        let remainder = (num % k + k) % k; // Ensure non-negative remainder
        let complement = (k - remainder) % k; // What remainder would pair with this one to sum to k or a multiple of k
        
        // Add how many times we've encountered the complement remainder before
        pair_count += mod_count[complement as usize];
        
        // Increment the count of the current remainder
        mod_count[remainder as usize] += 1;
    }

    pair_count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read n and k
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();
    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    // Read the array
    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Call the function and write the result to output
    let result = divisibleSumPairs(n, k, &ar);
    writeln!(&mut fptr, "{}", result).ok();
}
