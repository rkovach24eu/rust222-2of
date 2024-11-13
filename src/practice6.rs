fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

fn main() {
    let a = 48;
    let b = 18;
    println!("GCD of {} and {} is {}", a, b, gcd(a, b));
}
