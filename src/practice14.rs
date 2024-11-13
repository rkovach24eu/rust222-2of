#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn area_of_rectangle(rect: &Rectangle) -> i32 {
    let width = (rect.b.x - rect.a.x).abs();
    let height = (rect.a.y - rect.b.y).abs();
    width * height
}

// Функція для обчислення площі перетину двох прямокутників
fn intersection_area(r1: &Rectangle, r2: &Rectangle) -> i32 {
    let x_overlap = (r1.a.x.max(r2.a.x) - r1.b.x.min(r2.b.x)).max(0);
    let y_overlap = (r1.a.y.min(r2.a.y) - r1.b.y.max(r2.b.y)).max(0);

    x_overlap * y_overlap
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let n = xs.len();
    let mut total_area = 0;

    // Додаємо площу кожного прямокутника
    for i in 0..n {
        total_area += area_of_rectangle(&xs[i]);
    }

    // Віднімаємо площу перетинів
    for i in 0..n {
        for j in i + 1..n {
            total_area -= intersection_area(&xs[i], &xs[j]);
        }
    }

    total_area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    // Запуск тесту
    area_occupied_test();
    println!("Test passed!");
}

