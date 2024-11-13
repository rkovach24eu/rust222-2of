const SIZE: usize = 5;

fn main() {
    let mut output = String::new();
    
    // Верхня частина ромба
    for i in 0..SIZE {
        // Додаємо пробіли перед зірочками
        for _ in 0..(SIZE - i - 1) {
            output.push(' ');
        }
        // Додаємо зірочки з пробілами
        for _ in 0..(2 * i + 1) {
            output.push('*');
        }
        output.push('\n');
    }
    
    // Нижня частина ромба
    for i in (0..SIZE - 1).rev() {
        // Додаємо пробіли перед зірочками
        for _ in 0..(SIZE - i - 1) {
            output.push(' ');
        }
        // Додаємо зірочки з пробілами
        for _ in 0..(2 * i + 1) {
            output.push('*');
        }
        output.push('\n');
    }
    
    // Виводимо результат одним print!
    print!("{}", output);
}

