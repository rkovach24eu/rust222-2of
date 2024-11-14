use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    let mut rounded_grades = Vec::with_capacity(grades.len());
    
    for &grade in grades {
        if grade < 40 {
            // If grade is less than 40, no rounding occurs
            rounded_grades.push(grade);
        } else {
            // Find the next multiple of 5
            let next_multiple_of_5 = ((grade + 4) / 5) * 5;
            
            // If the difference is less than 5, round up
            if next_multiple_of_5 - grade < 5 {
                rounded_grades.push(next_multiple_of_5);
            } else {
                rounded_grades.push(grade);
            }
        }
    }

    rounded_grades
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    let result = gradingStudents(&grades);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
