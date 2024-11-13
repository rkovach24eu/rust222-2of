fn swap_case(input: &str) -> String {
    input.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}

fn main() {
    let text = "Hello, World!";
    let swapped_text = swap_case(text);
    println!("{}", swapped_text); // Виведе "hELLO, wORLD!"
}
