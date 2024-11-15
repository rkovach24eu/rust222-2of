use std::collections::HashMap;
use std::io::{self, BufRead};

fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut sock_counts = HashMap::new();

    // Count the frequency of each color
    for &sock in ar.iter() {
        *sock_counts.entry(sock).or_insert(0) += 1;
    }

    // Calculate the total number of pairs
    let mut pairs = 0;
    for &count in sock_counts.values() {
        pairs += count / 2; // Integer division gives the number of pairs
    }

    pairs
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read number of socks
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the list of sock colors
    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Call the function to calculate the number of pairs
    let result = sockMerchant(n, &ar);

    // Output the result
    println!("{}", result);
}
