use std::io::{self, BufRead};

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        // If the kangaroos are moving at the same speed, they will only meet if they start at the same position
        if x1 == x2 {
            return "YES".to_string();
        } else {
            return "NO".to_string();
        }
    }

    // For different speeds, check if the difference in positions is divisible by the difference in velocities
    if (x2 - x1) % (v1 - v2) == 0 && (x2 - x1) / (v1 - v2) >= 0 {
        return "YES".to_string();
    }

    "NO".to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let x1 = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let v1 = first_multiple_input[1].trim().parse::<i32>().unwrap();
    let x2 = first_multiple_input[2].trim().parse::<i32>().unwrap();
    let v2 = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = kangaroo(x1, v1, x2, v2);

    println!("{}", result);
}
