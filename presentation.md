---
defaultTemplate: "[[tpl_satlab]]"
---

# Rust Introduction

By Albert Laursen

note: 
1. Who am I?
2. Who is AAU Satlab?
3. What is the purpose of this presentation?
4. What study programmes are people from?
5. How much experience do you have with programming?

---

## Contents
- What is Rust
- Why Choose Rust
- C++/C vs Rust

- Installation
- Tooling
- Compiling

note: 
General structure of the presentation:
1. Introduction to Rust
2. Installation
3. Code examples

---

## What is Rust?
- Systems Programming Language
- Compiled
- Modern Language 
	- Type system, concurrency, memory safety
- No Runtime Garbage Collector
  - Scope & Lifetimes

note:
  1. Ask if people know what a systems programming language is
  2. Talk about the difference between compiled and interpreted languages
  3. What makes a language modern?
  4. What is a garbage collector?

---

## Why Choose Rust?
- Safety
  - No null pointers
  - Borrowing & Lifetimes
- Modern Tooling
  - Cargo & Crates
- Only modern choice for embedded systems
- Sizeable embedded community

note:
1. Why is safety important?
2. Why is a null pointer dangerous?
3. We will come back to borrowing and lifetimes
4. Cargo is THE tool for Rust
   1. Compare to the C ecosystem
5. Open Source vs Manufacturer Libraries
   1. Often times you have to use the manufacturer's libraries in C

---

## C++/C vs Rust

- C++/C is not safe
  - C++ is a superset of C
- Borrow Checker
  - Compile time safety
- Interfaces
  - Abstract classes vs Traits
  - e.g. `embedded-hal`
- OOP
  - C++:  `class`'es
  - Rust:  `struct`s, `enum`s & `impl`s

note:
1. C++ is not safe
   1. Null pointers
   2. Dangling pointers
   3. Buffer overflows
   4. Memory leaks
   5. Type confusion
2. Borrow Checker
   1. Practical example will be shown later
   2. Ownership, and lifetimes are assured at compile time
3. Interfaces
   1. Interfaces create a contract between the implementor and the user
   2. Traits don't require inheritance
   3. Some C++ interfaces are not actually defined in the language
4. OOP
   1. No inheritance in Rust
   2. Composition over inheritance
   3. Types and associated functions

---

## Further Reading
- Rust Book
  - Online Version: [doc.rust-lang.org/book](https://doc.rust-lang.org/book/)
  - Interactive Version: [rust-book.cs.brown.edu](https://rust-book.cs.brown.edu/)
- Rustlings Exercises
  - Interactive Lessons: [rust-lang/rustlings](https://github.com/rust-lang/rustlings)
- Cheatsheet
  - Cheat Sheet: [cheats.rs](https://cheats.rs/)

---

# Install Party
:partying_face:

---

## Installation
Go to [rustup.rs](https://rustup.rs) and follow the instructions

Or use the following command in your terminal:

Windows:
```powershell
winget install rustup
```

Linux/Mac:
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Tooling
- `rustc` - Rust Compiler
- `cargo` - Package Manager
- `rustup` - Rust Version Manager
- `rustfmt` - Code Formatter
- `clippy` - Linter

---

## Compiling

```bash
cargo new hello_world
cd hello_world
cargo build
cargo run
```
```text
Hello, world!
```

---

# Code Examples

Get git repository:
```bash
git clone https://github.com/Awlaursen/rust-examples.git
```

---

## Basic Syntax

File: `01-hello\src\main.rs`

```rust []½
// Primitive data types in Rust
let x: i32 = 5; // signed 32-bit integer
let y: f64 = 2.5; // 64-bit floating point
let z: u32 = 1_000_000; // unsigned 32-bit integer
let a: char = 'a'; // single Unicode character
let b: bool = true; // boolean
let c: &str = "Hello, world!"; // string slice
```

---
## Basic Syntax

File: `01-hello\src\main.rs`

```rust []
// inferred data types in Rust
let x = 5; // i32
let y = 2.5; // f64
```    

```rust [] 
// Explicitly specifying data types in Rust
let x = 5i32;
let y = 2.5f64;
let z = 1_000_000u32;
```

---

## Basic Syntax

File: `01-hello\src\main.rs`

```rust []
// Compound data types in Rust
let d = [1, 2, 3, 4, 5]; // [i32; 5]
let e = (1, 2, 3, 4, 5); // (i32, i32, i32, i32, i32)
```

```rust []
// Accessing elements in compound data types
let first = d[0];
let second = e.1;
print!("first = {}, second = {}", first, second);
```

---

## Basic Syntax

File: `01-hello\src\main.rs`
  
  ```rust []
  // Control flow in Rust
  if x < 5 {
      println!("x is less than 5");
  } else if x == 5 {
      println!("x is equal to 5");
  } else {
      println!("x is greater than 5");
  }
  ```

---

## Basic Syntax

File: `01-hello\src\main.rs`

<split even gap=1>

  ```rust []
  // Loops in Rust
  for i in 0..5 {
      println!("i = {}", i);
  }

  let mut i = 0;
  while i < 5 {
      println!("i = {}", i);
      i += 1;
  }
  ```

  ```rust []
  // Loop forever until break
  let mut i = 0;
  loop {
      println!("i = {}", i);
      i += 1;
      if i == 5 {
          break;
      }
  }
  ```

</split>

---

## Basic Syntax

File: `01-hello\src\main.rs`

  ```rust []
  // Mutability in Rust
  let mut x = 5;
  x = 10;
  ```
    
  ```rust []
  // Shadowing in Rust
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  ```

---

## Basic Syntax

File: `01-hello\src\main.rs`

  ```rust []
  // Macros in Rust
  let my_vector = vec![1, 2, 3, 4, 5]; // Vec<i32>
  println!("x = {}, y = {}, z = {}", x, y, z);

  println!("{:?}", my_vector);
  ```

  ```rust []
  let my_vector = <[_]>::into_vec(
      #[rustc_box]
      alloc::boxed::Box::new([1, 2, 3, 4, 5]),
  ); // Vec<i32>

  std::io::_print(
    builtin #format_args("x = {}, y = {}, z = {}\n",x,y,z));

  ```