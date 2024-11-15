use std::io::{self, BufRead, Write};
use std::env;
use std::fs::File;

fn simpleArraySum(ar: &[i32]) -> i32 {
    // Using iterator and sum method to sum the elements of the array
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the size of the array (though it's not needed for the sum calculation)
    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the array elements
    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Call the function to get the sum
    let result = simpleArraySum(&ar);

    // Write the result to the file or stdout
    writeln!(&mut fptr, "{}", result).ok();
}

