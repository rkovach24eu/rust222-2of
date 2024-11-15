use std::io::{self, BufRead};
use std::cmp::min;

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        return x;
    }
    gcd(y, x % y)
}

fn lcm(x: i32, y: i32) -> i32 {
    (x * y) / gcd(x, y)
}

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    // Find the LCM of all elements in array a
    let mut lcm_a = a[0];
    for &num in &a[1..] {
        lcm_a = lcm(lcm_a, num);
    }

    // Find the GCD of all elements in array b
    let mut gcd_b = b[0];
    for &num in &b[1..]
