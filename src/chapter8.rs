#[test]
fn test1() {
    // Fill the blanks
    enum Direction {
        East,
        West,
        North,
        South,
    }
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => { // Matching South or North here
            println!("South or North");
        },
        _ => println!("West"),
    };
}

#[test]
fn test2() {
    let boolean = true;
    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    assert_eq!(binary, 1);
    println!("Success!");
}

#[test]
fn test3() {
    // Fill in the blanks
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];
    for msg in msgs {
        show_message(msg)
    }
    fn show_message(msg: Message) {
        match msg {
            Message::Move{x: a, y: b} => { // match  Message::Move
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            },
            Message::ChangeColor(_r, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            __ => println!("no data in these variants")
        }
    }
    println!("Success!");
}

#[test]
fn test4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];
    // Fill the blank with matches! to make the code work
    for ab in alphabets {
        assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'));
    }
    println!("Success!");
}

#[test]
fn test5() {
    enum MyEnum {
        Foo,
        Bar
    }
    let mut count = 0;
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) { // Fix the error by changing only this line
            count += 1;
        }
    }
    assert_eq!(count, 2);
    println!("Success!");
}

#[test]
fn test6() {
    let o = Some(7);
    // Remove the whole match block, using if let instead
    if let Some(i) = o {
        println!("This is a really long string and {:?}", i);
        println!("Success!");
    }
}

#[test]
fn test7() {
    // Fill in the blank
    enum Foo {
        Bar(u8)
    }
    let a = Foo::Bar(1);
    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);
        println!("Success!");
    }
}

#[test]
fn test8() {
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }
    let a = Foo::Qux(10);
    // Remove the codes below, using match instead
    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ => println!("match others"),
    }
}

#[test]
// Fix the errors in-place
fn test9() {
    let age = Some(30);
    if let Some(age) = age { // Create a new variable with the same name as previous age
        assert_eq!(age, 30);
    } // The new variable age goes out of scope here
    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
}

#[test]
fn test10() {
    match_number(1);
} // No output
fn match_number(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        // Fill in the blank with |, DON'T use .. or ..=
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        // Match an inclusive range
        6..=10 => {
            println!("match 6 -> 10")
        }
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}

#[test]
fn test11() {
    struct Point {
        x: i32,
        y: i32,
    }
    // Fill in the blank to let p match the second arm
    let p = Point { x: 3, y: 30 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Second arm
        Point {
            x: 0..=5,
            y: y@ (10 | 20 | 30)
        } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

#[test]
fn test12() {
    // Fix the errors
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id @ 3..=7,
        } => println!("Found an id in range [3, 7]: {}", id),
        Message::Hello { id: newid @ (10 | 11 | 12),
        } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

#[test]
// Fill in the blank to make the code work, split MUST be used
fn test13() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }
    println!("Success!");
}

#[test]
fn test14() {
    // Fill the blank to make the code work
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }
    println!("Success!");
}

#[test]
fn test15() {
    // FIX the error with least changing
    // DON'T remove any code line
    let mut v = String::from("hello,");
    let r = &mut v;
    match r {
        value => value.push_str(" world!")
    }
}
