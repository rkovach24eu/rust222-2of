#[test]
fn test1() {
    // FILL in the blanks and FIX errors
    // 1. Don't use `to_string()`
    // 2. Don't add/remove any code line
    let mut s: String = String::from ("hello, ");
    s.push_str("world");
    s.push('!');


    move_ownership(s.clone());


    assert_eq!(s, "hello, world!");


    println!("Success!");
}


fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}

#[test]
fn test2() {
    // FILL in the blanks
    let mut s: String = String::from("hello, world");


    let slice1: &str = s.as_str(); // In two ways
    assert_eq!(slice1, "hello, world");


    let slice2: &str = &s[..5];
    assert_eq!(slice2, "hello");


    let slice3: &mut String = &mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");


    println!("Success!");
}

#[test]
fn test3() {
    // Question: how many heap allocations are happening here?
    // Your answer:
    // Create a String type based on `&str`
    // The type of string literals is `&str`
    let s: String = String::from("hello, world!"); //1


    // Create a slice point to String `s`
    let slice: &str = &s; //"hello, world!


    // Create a String type based on the recently created slice
    let s: String = slice.to_string(); //2


    assert_eq!(s, "hello, world!");


    println!("Success!");
}

#[test]
fn test4() {
    // FILL in the blank and FIX errors
    let s: String = String::from("hello, 世界");
    let slice1: &str = &s[..1]; //tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");


    let slice2: &str = &s[7..10]; // Tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(slice2, "世");


    // Iterate through all chars in s
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }


    println!("Success!");
}

#[test]
fn test5() {
    // FILL in the blanks
    let mut s: String = String::new(); //Vec<u8> - > "hello"
    s.push_str("hello");


    // Some bytes, in a vector
    let v: Vec<u8> = vec![104, 101, 108, 108, 111];


    // Turn a byte's vector into a String
    let s1: String = String::from_utf8(v).unwrap();




    assert_eq!(s, s1); //"hello"


    println!("Success!");
}

#[test]
fn test6() {
    // Modify the code below to print out:
    // 25
    // 25
    // 25
    // Here, there’s no need to allocate more memory inside the loop.
    let mut s: String = String::with_capacity(25);


    println!("{}", s.capacity());


    for _ in 0..2 {
        s.push_str("hello"); //"hello"
        println!("{}", s.capacity());
    }
    println!("{}", s);


    println!("Success!");
}

use std::mem;
#[test]
fn test7() {
    // FILL in the blanks
    let story = String::from("Rust By Practice");


    // Prevent automatically dropping the String's data
    let mut story = mem::ManuallyDrop::new(story);


    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();


    // story has nineteen bytes
    assert_eq!(16, len);


    // We can re-build a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };


    assert_eq!(*story, s);


    println!("Success!")
}

#[test]
fn test8() {
    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) and vec![..] are same macros, so
    let v = vec!(1, 2, 3);
    is_vec(&v);

    // in code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE `for` to rewrite the below code
    let mut v1 = Vec::new();
    for i in &v {
        v1.push(*i)
    }
    is_vec(&v1);

    assert_eq!(format!("{:?}",v), format!("{:?}",v1));

    println!("Success!")
}

fn is_vec(v: &Vec<u8>) {}

#[test]
fn test9() {
    let mut v1: Vec<i32> = Vec::from([1, 2, 4]);
    v1.pop(); //[1,2]
    v1.push(3); //[1,2,3]

    let mut v2: Vec<i32>  = Vec::new();
    v2.extend(&v1);

    assert_eq!(v1, v2);

    println!("Success!");
}

#[test]
fn test10() {
    // FILL in the blanks
    // Array -> Vec
    // impl From<[T; N]> for Vec
    let arr: [i32; 3] = [1, 2, 3];
    let v1: Vec<i32> = Vec::from(arr);
    let v2: Vec<i32> = arr.into();

    assert_eq!(v1, v2);

    // String -> Vec
    // impl From<String> for Vec
    let s: String = "hello".to_string(); //Vec<u8>
    let v1: Vec<u8> = s.into_bytes();

    let s: String = "hello".to_string();
    let v2: Vec<u8> = s.into_bytes();
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s: &str = "hello";
    let v3: Vec<u8> = Vec::from(s);
    assert_eq!(v2, v3);

    // Iterators can be collected into vectors
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]); //[0,0,0, ...,0]

    println!("Success!");
}


//4
// FIX the error and IMPLEMENT the code
#[test]
fn test11() {
    let mut v = Vec::from([1, 2, 3]);

    for i in 0..5 {
        println!("{:?}", v.get(i)) //Option<i32>
    }

    for i in 0..5 {
        match v.get(i) {
            Some(e) => v[i] = e + 1,
            None => v.push(i + 2)
        }
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}

#[test]
fn test12() {
    // FIX the errors
    let mut v: Vec<i32> = vec![1, 2, 3];

    let slice1: &[i32] = &v[..];
    // Out of bounds will cause a panic
    // You must use `v.len` here
    let slice2: &[i32] = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    // Slices are read only
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3: &[i32] = &v[0..];

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!");
}

#[test]
fn test13() {
    // FIX the errors
    let mut vec = Vec::with_capacity(10);

    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    // ...but this may make the vector reallocate
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    // Fill in an appropriate value to make the `for` done without reallocating
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("Success!");
}

#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}
#[test]
fn test14() {
    // FILL in the blank
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    // Comparing two enums need to derive the PartialEq trait
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!")
}
trait IpAddr1{
    fn display(&self);
}

struct V4(String);
impl IpAddr1 for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr1 for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}
#[test]
fn test15() {
    let v: Vec<Box<dyn IpAddr1>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}

#[test]
use std::collections::HashMap;
fn test16() {
    // FILL in the blanks and FIX the errors
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    // Get returns an Option<&V>
    let score: Option<&i32> = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        // Indexing returns a value V
        let score: i32 = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score);
    }
}

#[test]
fn test17() {
    let teams: [(&str, i32); 3] = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1: HashMap<&str, i32> = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }
    // IMPLEMENT team_map2 in two ways
    // Tips: one of the approaches is to use `collect` method
    let teams_map2: HashMap<&str, i32> = HashMap::from(teams);

    assert_eq!(teams_map1, teams_map2);

    println!("Success!");
}

#[test]
fn test18() {
    // FILL in the blanks
    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();

    // Insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);

    // Insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    // Ensures a value is in the entry by inserting the default if empty, and returns
    // a mutable reference to the value in the entry.
    let health: &mut u8 = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("Success!");
}

fn random_stat_buff() -> u8 {
    // Could actually return some random value here - let's just return
    // some fixed value for now
    42
}

// FIX the errors
// Tips: `derive` is usually a good way to implement some common used traits
#[derive(Debug, Hash, Eq, PartialEq)]

struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}
#[test]
fn test19() {
    // Use a HashMap to store the vikings' health points.
    let vikings: HashMap<Viking, i32> = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}

#[test]
fn test20() {
    // FIX the errors with least changes
    // DON'T remove any code line
    let v1: i32 = 10;
    let mut m1: HashMap<i32, i32> = HashMap::new();

    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    let v2: String = "hello".to_string();
    let mut m2: HashMap<&str, i32> = HashMap::new();
    // Ownership moved here
    m2.insert(&v2, v1);

    assert_eq!(v2, "hello");

    println!("Success!");
}
