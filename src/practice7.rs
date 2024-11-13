const TRIANGLE_COUNT: usize = 4; // Кількість трикутників у ялинці

fn main() {
    let mut output = String::new();
    
    // Створюємо ялинку з вказаною кількістю трикутників
    (1..=TRIANGLE_COUNT).for_each(|i| {
        let triangle_height = i + 2; // Висота кожного трикутника збільшується
        (0..triangle_height).for_each(|row| {
            // Додаємо пробіли для центрованого вигляду
            output.push_str(&" ".repeat(TRIANGLE_COUNT + triangle_height - row - 1));
            // Додаємо зірочки для побудови трикутника
            output.push_str(&"*".repeat(2 * row + 1));
            output.push('\n');
        });
    });

    // Створюємо стовбур ялинки
    (0..TRIANGLE_COUNT).for_each(|_| {
        output.push_str(&" ".repeat(TRIANGLE_COUNT + 1));
        output.push_str("***\n");
    });

    // Виводимо результат
    print!("{}", output);
}
