use std::io::{self, BufRead, Write};
use std::env;
use std::fs::File;

fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    // Find the tallest candle by finding the maximum value in the array
    let tallest = *candles.iter().max().unwrap();
    
    // Count how many times the tallest candle appears in the array
    candles.iter().filter(|&&x| x == tallest).count() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the number of candles (though this is not needed in the function itself)
    let _candles_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the candle heights into a vector
    let candles: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Call the birthdayCakeCandles function to get the result
    let result = birthdayCakeCandles(&candles);

    // Write the result to the output
    writeln!(&mut fptr, "{}", result).ok();
}
