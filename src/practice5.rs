const WIDTH: usize = 10; // Ширина конверта
const HEIGHT: usize = 5; // Висота конверта

fn main() {
    let mut output = String::new();

    // Верхня частина конверта (кришка)
    for i in 0..HEIGHT {
        // Додаємо пробіли перед діагональними лініями
        for _ in 0..i {
            output.push(' ');
        }
        // Додаємо діагональні лінії
        output.push('/');
        for _ in 0..(2 * (HEIGHT - i) - 1) {
            output.push(' ');
        }
        output.push('\\');
        output.push('\n');
    }

    // Нижня частина конверта (тіло)
    output.push('+');
    for _ in 0..(2 * HEIGHT - 1) {
        output.push('-');
    }
    output.push('+');
    output.push('\n');
    
    for _ in 0..WIDTH - HEIGHT {
        output.push('|');
        for _ in 0..(2 * HEIGHT - 1) {
            output.push(' ');
        }
        output.push('|');
        output.push('\n');
    }

    output.push('+');
    for _ in 0..(2 * HEIGHT - 1) {
        output.push('-');
    }
    output.push('+');
    output.push('\n');

    // Виводимо результат
    print!("{}", output);
}
