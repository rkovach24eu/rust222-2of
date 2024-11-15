use std::io::{self, BufRead};

fn a_very_big_sum(ar: &[i64]) -> i64 {
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let ar: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let result = a_very_big_sum(&ar);
    println!("{}", result);
}

