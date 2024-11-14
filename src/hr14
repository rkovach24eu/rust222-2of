use std::io::{self, BufRead};

fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    // Initialize the variables
    let mut min = scores[0];  // First game score is the initial minimum
    let mut max = scores[0];  // First game score is the initial maximum
    let mut min_breaks = 0;   // Counter for breaking min record
    let mut max_breaks = 0;   // Counter for breaking max record
    
    // Process the scores from the second game onwards
    for &score in &scores[1..] {
        if score > max {
            max = score;      // Update max
            max_breaks += 1;  // Increment max record breaks
        }
        if score < min {
            min = score;      // Update min
            min_breaks += 1;  // Increment min record breaks
        }
    }
    
    // Return the results as a vector of two integers: [max_breaks, min_breaks]
    vec![max_breaks, min_breaks]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    
    let _ = stdin_iterator.next(); // Read the number of games, not needed in our logic
    
    // Read the scores
    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Call the breakingRecords function and get the result
    let result = breakingRecords(&scores);
    
    // Print the result
    println!("{} {}", result[0], result[1]);
}

