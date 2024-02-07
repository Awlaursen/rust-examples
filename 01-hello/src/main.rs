
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

    // Compound data types in Rust
    let d = [1, 2, 3, 4, 5]; // [i32; 5]
    let e = (1, 2, 3, 4, 5); // (i32, i32, i32, i32, i32)

    // Accessing elements in compound data types
    let first = d[0];
    let second = e.1;
    print!("first = {}, second = {}", first, second);

    // Mutability in Rust
    let mut x = 5;
    x = 10;

    // Macros in Rust
    let my_vector = vec![1, 2, 3, 4, 5]; // Vec<i32>
    println!("
    x = {}
    y = {}
    z = {}
    a = {}
    b = {}
    c = {}
    d = {:?}
    e = {:?}", x, y, z, a, b, c, d, e);

    println!("{:?}", my_vector);
}
