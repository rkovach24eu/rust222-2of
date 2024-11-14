use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn migratoryBirds(arr: &[i32]) -> i32 {
    // Initialize an array to count the frequency of each bird type (1-5)
    let mut counts = vec![0; 5];

    // Count the frequency of each bird type
    for &bird in arr {
        counts[(bird - 1) as usize] += 1; // bird type 1 corresponds to index 0, etc.
    }

    // Find the bird type with the highest frequency (and the smallest type if there are ties)
    let mut max_count = 0;
    let mut bird_type = 0;

    for (i, &count) in counts.iter().enumerate() {
        if count > max_count {
            max_count = count;
            bird_type = i + 1; // Add 1 to index to get the bird type id
        }
    }

    bird_type
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the number of bird sightings (we don't actually need to store this value)
    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the bird sightings into a vector
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Call the migratoryBirds function
    let result = migratoryBirds(&arr);

    // Write the result to the output
    writeln!(&mut fptr, "{}", result).ok();
}
