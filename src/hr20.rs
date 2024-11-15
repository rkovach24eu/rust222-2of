use std::io::{self, BufRead};

fn pageCount(n: i32, p: i32) -> i32 {
    // Calculate the number of flips from the front
    let front_turns = p / 2;

    // Calculate the number of flips from the back
    let back_turns = (n / 2) - (p / 2);

    // Return the minimum of the two
    std::cmp::min(front_turns, back_turns)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read number of pages in the book
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the target page number
    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Call the pageCount function and print the result
    let result = pageCount(n, p);
    println!("{}", result);
}
