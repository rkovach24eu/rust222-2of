fn find_variables() -> (char, char, char, char, char, char, char, char) {
    // Генерація всіх можливих комбінацій букв для змінних m, u, x, a, s, l, o, n
    let possible_values = vec!['m', 'u', 'x', 'a', 's', 'l', 'o', 'n'];
    
    // Перебираємо всі можливі комбінації
    for m in &possible_values {
        for u in &possible_values {
            for x in &possible_values {
                for a in &possible_values {
                    for s in &possible_values {
                        for l in &possible_values {
                            for o in &possible_values {
                                for n in &possible_values {
                                    if *m != *u && *m != *x && *m != *a && *m != *s && *m != *l && *m != *o && *m != *n &&
                                       *u != *x && *u != *a && *u != *s && *u != *l && *u != *o && *u != *n &&
                                       *x != *a && *x != *s && *x != *l && *x != *o && *x != *n &&
                                       *a != *s && *a != *l && *a != *o && *a != *n &&
                                       *s != *l && *s != *o && *s != *n &&
                                       *l != *o && *l != *n &&
                                       *o != *n {
                                        return (*m, *u, *x, *a, *s, *l, *o, *n);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Якщо рішення не знайдено
    (' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ')
}

fn print_result(m: char, u: char, x: char, a: char, s: char, l: char, o: char, n: char) {
    println!("{}{}{}", m, u, x, a);
    println!("{}        {}", x, a);
    println!("  ------");
    println!("    {}{}", s, l, o, n);
}

fn main() {
    let (m, u, x, a, s, l, o, n) = find_variables();
    
    if m != ' ' {
        print_result(m, u, x, a, s, l, o, n);
    } else {
        println!("No solution found.");
    }
}
