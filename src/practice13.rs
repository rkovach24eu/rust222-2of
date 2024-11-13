use rand::Rng;

// Функція перевірки, чи можна рівномірно розподілити вантаж
fn can_distribute_evenly(shipments: &Vec<u32>, n_ships: usize) -> bool {
    let total: u32 = shipments.iter().sum();
    total % n_ships as u32 == 0
}

// Функція, яка рахує мінімальну кількість переносу вантажу
fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n_ships = shipments.len();

    // Якщо неможливо розподілити рівномірно, повертаємо 0
    if total % n_ships as u32 != 0 {
        return 0;
    }

    let target = total / n_ships as u32;
    let mut moves = 0;
    let mut excess = 0;

    for &shipment in shipments.iter() {
        let diff = shipment as i32 - target as i32;
        excess += diff;

        if excess > 0 {
            moves += excess;
        }
    }

    moves as usize
}

// Функція генерації Vec<u32> з вантажами, які можуть бути рівномірно розподілені
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut shipments = Vec::new();
    let total_cargo: u32 = rng.gen_range(100..200) * n as u32;  // Генеруємо загальну кількість вантажу

    // Генеруємо рівномірно розподілені вантажі
    let base = total_cargo / n as u32;
    for _ in 0..n {
        shipments.push(base);
    }

    shipments
}

fn main() {
    // Приклад генерації вектора вантажів для 5 кораблів
    let shipments = gen_shipments(5);
    println!("Generated Shipments: {:?}", shipments);

    // Перевірка, чи можна рівномірно розподілити вантаж
    let n_ships = shipments.len();
    if can_distribute_evenly(&shipments, n_ships) {
        println!("The cargo can be distributed evenly.");
    } else {
        println!("The cargo cannot be distributed evenly.");
    }

    // Підрахуємо мінімальні переміщення вантажу
    let moves = count_permutation(&shipments);
    println!("Minimum number of moves to balance the cargo: {}", moves);
}
