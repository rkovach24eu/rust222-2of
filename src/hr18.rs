use std::io::{self, BufRead};

fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    // Calculate the total cost of the bill
    let total_cost: i32 = bill.iter().sum();

    // Subtract the item Anna didn't eat
    let anna_share = (total_cost - bill[k as usize]) / 2;

    // Check if Brian charged Anna the correct amount
    if b == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna_share); // Print the overcharge amount
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read input values
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap(); // number of items
    let k = first_multiple_input[1].trim().parse::<i32>().unwrap(); // index of the item Anna didn't eat

    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect(); // bill items

    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap(); // amount Brian charged Anna

    bonAppetit(&bill, k, b);
}
