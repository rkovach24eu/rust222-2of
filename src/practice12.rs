use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100) as i32).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<i32> {
    if data.len() < 2 {
        return None; // Якщо вектор менший за 2 елементи, неможливо знайти пару
    }

    let mut min_sum = data[0] + data[1];

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
        }
    }

    Some(min_sum)
}

fn print_vector(data: &[i32]) {
    println!("Vector elements:");
    for (index, &value) in data.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }
}

fn main() {
    // Генерація випадкового вектора довжиною 20
    let random_vector = gen_random_vector(20);
    println!("Generated Random Vector:");
    print_vector(&random_vector);
    
    // Знаходимо мінімальну пару сусідніх елементів
    match min_adjacent_sum(&random_vector) {
        Some(min) => println!("The minimum adjacent sum is: {}", min),
        None => println!("The vector is too short to find adjacent pairs."),
    }
}
