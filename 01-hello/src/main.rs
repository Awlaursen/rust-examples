// Overview of Rust data types and basic syntax

fn main() {
    println!("Hello, world!");

    // Primitive data types in Rust
    let x: i32 = 5; // signed 32-bit integer
    let y: f64 = 2.5; // 64-bit floating point
    let z: u32 = 1_000_000; // unsigned 32-bit integer
    let a: char = 'a'; // single Unicode character
    let b: bool = true; // boolean
    let c: &str = "Hello, world!"; // string slice

    // inferred data types in Rust
    let x = 5; // i32
    let y = 2.5; // f64

    // Explicitly specifying data types in Rust
    let x = 5i32;
    let y = 2.5f64;
    let z = 1_000_000u32;

    // Compound data types in Rust
    let d = [1, 2, 3, 4, 5]; // [i32; 5]
    let e = (1, 2, 3, 4, 5); // (i32, i32, i32, i32, i32)

    // Accessing elements in compound data types
    let first = d[0];
    let second = e.1;
    print!("first = {}, second = {}", first, second);

    // Control flow in Rust
    if x < 5 {
        println!("x is less than 5");
    } else if x == 5 {
        println!("x is equal to 5");
    } else {
        println!("x is greater than 5");
    }

    // Loops in Rust
    for i in 0..5 {
        println!("i = {}", i);
    }

    let mut i = 0;
    while i < 5 {
        println!("i = {}", i);
        i += 1;
    }

    i = 0;
    loop {
        println!("i = {}", i);
        i += 1;
        if i == 5 {
            break;
        }
    }

    // Mutability in Rust
    let mut x = 5;
    x = 10;

    // Shadowing in Rust
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    // Macros in Rust
    let my_vector = vec![1, 2, 3, 4, 5]; // Vec<i32>
    println!("x = {}, y = {}, z = {}", x, y, z);

    println!("{:?}", my_vector);
    
}
