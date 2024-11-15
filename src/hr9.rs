use std::io::{self, BufRead, Write};
use std::env;
use std::fs::File;

fn timeConversion(s: &str) -> String {
    // Extract the time period (AM/PM)
    let period = &s[8..];  // AM or PM
    // Extract the hour, minute, second from the input time string
    let (hour_str, minute_str, second_str) = s.split_at(2);
    let minute_second = &minute_str[1..];

    let hour: i32 = hour_str.parse().unwrap();
    let (minute, second) = minute_second.split_at(2);

    // Convert to military time (24-hour format)
    let military_hour = if period == "AM" {
        if hour == 12 {
            0 // 12 AM is 00 in 24-hour format
        } else {
            hour // AM times remain the same unless it's 12 AM
        }
    } else {
        if hour == 12 {
            12 // 12 PM stays as 12
        } else {
            hour + 12 // PM times, add 12 to convert to 24-hour format
        }
    };

    // Format the hour back to two digits (e.g., "03" instead of "3")
    format!("{:02}:{:02}:{:02}", military_hour, minute.parse::<i32>().unwrap(), second.parse::<i32>().unwrap())
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
