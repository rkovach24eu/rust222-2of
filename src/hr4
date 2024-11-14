use std::io::{self, BufRead};

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += arr[i][i];
        secondary_diagonal_sum += arr[i][n - 1 - i];
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n);

    for _ in 0..n {
        let row: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        arr.push(row);
    }

    let result = diagonal_difference(&arr);
    println!("{}", result);
}
