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
- Getting Started

note: 
General structure of the presentation:
1. Introduction to Rust
2. Installation
3. Code examples

---

## What is Rust?
- Systems Programming Language
+ Compiled
+ Modern Language 
	- Type system, concurrency, memory safety
+ No Runtime Garbage Collector
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
+ Modern Tooling
  - Cargo & Crates
+ Only modern choice for embedded systems
+ Sizeable embedded community

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
+ Borrow Checker
  - Compile time safety
+ Interfaces
  - Abstract classes vs Traits
  - e.g. `embedded-hal`
+ OOP
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

## Visual Studio Code

::: block <!-- element style="font-size: 0.7em;" -->

| Extension        | Description                                | Rec             |
| ---------------- | ------------------------------------------ | --------------- |
| rust-analtzer    | Language Server, Formatter, Linter         | :green_square:  |
| Even Better TOML | TOML language support, primarily for cargo | :green_square:  |
| CodeLLDB         | Debugger for Host applications             | :green_square:  |
| Dependi          | Dependency Tool for cargo config           | :yellow_square: |
| Error Lens       | Better Error Highlightning                 | :yellow_square: |

:::

---

# Code Examples

Get git repository:
```bash
git clone https://github.com/Awlaursen/rust-examples.git
```

---

## Basic Syntax

File: `01-hello\src\main.rs`

```rust []
// Primitive data types in Rust
let x: i32 = 5; // signed 32-bit integer
let y: f64 = 2.5; // 64-bit floating point
let z: u32 = 1_000_000; // unsigned 32-bit integer
let a: char = 'a'; // single Unicode character
let b: bool = true; // boolean
let c: &str = "Hello, world!"; // string slice
```

note:
1. Types are on the right side, compared to C
2. `i32` is the default integer type
3. Mention `_` for readability
4. `&str` is a string slice, we will come back to this later


---
## Basic Syntax

File: `01-hello\src\main.rs`

```rust []
// inferred data types in Rust
let x = 5; // i32
let y = 2.5; // f64
let z = 1_000_000; // i32
```    

```rust [] 
// Explicitly specifying data types in Rust
let x = 5i32;
let y = 2.5f64;
let z = 1_000_000u32;
```

note:
1. When iferred, z will be an `i32` because it is the default
   1. Unless z is passed to a function that requires a `u32`
2. Explicitly specifying is not necessary but can be useful in some cases
   1. It's similar to C but with more specific types
---

## Basic Syntax

File: `01-hello\src\main.rs`

```rust []
// Compound data types in Rust
let d = [1, 2, 3, 4, 5]; // [i32; 5]
let e = (1, 2, 3, 4.5, 5.5); // (i32, i32, i32, f64, f64)
```

```rust []
// Accessing elements in compound data types
let first = d[0];
let second = e.1;
print!("first = {}, second = {}", first, second);
```

note:
1. Arrays and tuples similar but different
   1. Tuples can have different types
2. Tuples are often used for anonymous structs

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

note:
1. Rust has very cool control flow that we will see more of later
   1. `match` 
   2. `if let` 
   3. `while let` 

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

note:
1. `loop` is the same as `while true`
   1. Very useful for embedded systems

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

note:
1. Mutability is not the same as reassignment
2. Shadowing is useful for changing types
   1. `let x = 5;` `let x = "hello";`
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

note:
1. Macros are replaced at compile time
2. Helps with tedious ownership and borrowing rules
   1. This will become more clear later
3. Not necessary to understand how the generated code works
   1. The whole reason for macros is to avoid writing this code

---

## Functions

File: `02-functions\src\main.rs`

```rust []
// Function with no return value
fn print_result(result: i32) {
    println!("result is: {}", result);
}

// Being explicit about return value
fn print_result(result: i32) -> () {
    println!("result is: {}", result);
    return (); // Or just ()
}
```

note:
1. Unit type `()` is the same as `void` in C
2. Implicit return happens if there is no semicolon at the end of the function
   1. This is an expression instead of a statement

---

## Functions

File: `02-functions\src\main.rs`

```rust []
// Function with return value
fn add_numbers(x: i32, y: i32) -> i32 {
    return x + y;
}

// Function with multiple return values
fn add_and_multiply(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}
```

note:
1. Rust does not actually return multiple values but a tuple
2. What happens if `x` and `y` are `i64`?
3. What happens if `x` and `y` are different types? 

---

## Ownership

File: `03-ownership\src\bin\01-ownership.rs`

```rust []
// Scopes are defined by blocks like this one or functions
{                       // s is not valid here, not yet declared
    let s = "hello";    // s is valid from this point forward
    
    // do stuff with s
    println!("{}", s);
}                       // this scope is now over, and s is no 
                        // longer valid
println!("{}", s); // This will not compile
```

note:
1. Try to add the line `let s = "howdy";` before the block 

---

## Ownership

```text
error[E0425]: cannot find value `s` in this scope
  --> src\bin\01-ownership.rs:10:20
   |
 9 |     println!("{}", s);
   |                    ^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `structs` (bin "01-ownership") due to 1
       previous error
```

note:
1. running `cargo check` will give you the same error message
2. Error Lens will try to compress the error message

---

## Ownership

File: `03-ownership\src\bin\01-ownership.rs`

```rust []
// Move of ownership from s1 to s2
let s1 = String::from("hello");
let s2 = s1;  // Move happens because String is a heap-allocated 
              // type and it is not a `Copy` type

println!("{s1}, world!"); // This will not compile
```

note:
1. What vulnerability does this prevent?
   1. Double free
   2. Use after free
2. Why does this matter for embedded systems with no heap?
   1. Ownership is still important for stack-allocated types
   2. resources like peripherals
   3. memory-mapped registers

---

## Ownership

```text
error[E0382]: borrow of moved value: `s1`
  --> src\bin\01-ownership.rs:17:15
   |
13 |     let s1 = String::from("hello");
   |         -- move occurs because `s1` has type `String`, which 
                              does not implement the `Copy` trait
14 |     let s2 = s1;    
   |              -- value moved here
...
17 |     println!("{s1}, world!"); 
   |               ^^^^ value borrowed here after move
   |
```
```text
help: consider cloning the value if the performance cost is ok
   |
14 |     let s2 = s1.clone();    
   |                ++++++++
``` 

---

## Ownership

File: `03-ownership\src\bin\01-ownership.rs`

```rust []
// Clone of s3 to s4
    let s3 = String::from("hello");
    let s4 = s3.clone();    // Clone is called explicitly to create a deep copy

    println!("{}, world!", s3); // This will compile
```

note:
1. Difference between `clone` and `copy`
2. What is the performance cost of `clone`?
3. What is the performance cost of `copy`?

---

## Ownership

File: `03-ownership\src\bin\01-ownership.rs`

```rust []
// Ownership and functions
let s = String::from("hello"); // s comes into scope

takes_ownership(s);   // s's value moves into the function...
                      // ... and so is no longer valid here

let x = 5;            // x comes into scope

makes_copy(x);        // x would move into the function,
                      // but i32 is Copy, so it's okay still
                      // use x afterward
```

---
## Ownership

File: `03-ownership\src\bin\01-ownership.rs`

```rust []	
fn takes_ownership(s: String) { // s comes into scope 
    println!("{}", s);          
} // Here, s goes out of scope and `drop` is called. 

fn makes_copy(i: i32) { // i comes into scope
    println!("{}", i);
} // Here, i goes out of scope. Nothing special happens.

```

note:
1. Why is `String` `Take` and `i32` `Copy`?
2. What does `drop` do?
3. Does this mean that `String`s can never be used in functions? 

---

## Ownership

File: `03-ownership\src\bin\01-ownership.rs`

```rust []
let s2 = String::from("hello");     // s2 comes into scope

let s3 = takes_gives_back(s2);  // s2 is moved into
                                // takes_gives_back, which also
                                // moves its return value into s3

// This function takes a String and returns one
fn takes_gives_back(s: String) -> String {  // s comes into scope
  println!("{}", s); 
  s  // s is returned and moves out to the calling function
}
```

note:
1. This allows us to reuse the original string
2. It seems very tedious to have to pass around ownership all the time

---

## Borrowing

File: `03-ownership\src\bin\02-borrowing.rs`

| Syntax | Description |
|--------|-------------|
| `&T` | immutable reference |
| `&mut T` | mutable reference |
| `&str` | string slice |
| `&[T]` | slice |

note:
1. Rules:
   1. Only 1 mutable reference
   2. Multiple immutable references
   3. No mutable and immutable references at the same time

Memoy Layout:
```text	
&T  : (pointer)          Thin Pointer
&[T]: (pointer, length)  Fat Pointer, length = number of elements
$str: (pointer, length)  Fat Pointer, length = number of bytes
``` 
---

## Borrowing

File: `03-ownership\src\bin\02-borrowing.rs`

```rust []
let s1 = String::from("hello");

// Function takes an immutable reference
let len = calculate_length(&s1);

println!("The length of '{s1}' is {len}.");

fn calculate_length(s: &String) -> usize {
    // reference is {read-only / immutable / borrowed}
    s.len() // returns the length of the string without 
            // needing ownership
}
```

note:
1. You can always see what a function expects by looking at the function signature
   2. `String::len` takes an immutable reference to `self`

---

## Borrowing

File: `03-ownership\src\bin\02-borrowing.rs`

```rust []
// This block will not compile
let s = String::from("hello");

// Function takes an immutable reference
// but tries to mutate the borrowed data
change(&s);

fn change(some_string: &String) {
    // mutability is not allowed for borrowed references
    some_string.push_str(", world"); // This will not compile
}
```

note:
1. Look at the `String::push_str` function signature
   1. What does it take as an argument?
2. Take a look at the error message
3. What do we need to change in the function signature?

---

## Borrowing

File: `03-ownership\src\bin\02-borrowing.rs`

```rust []
// This block will not compile
let mut s = String::from("hello");

// There can be only one mutable reference to a
// particular piece of data at a time
let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

note:
1. What is the error message?
2. Does anyone know the actual solution to this problem?
   1. Interior mutability with `Cell` and `RefCell`
      1. Mutex and RwLock is also a solution
   2. We wont go into this in this presentation

---

## Borrowing

File: `03-ownership\src\bin\02-borrowing.rs`

```rust []
// This block will compile
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new 
  // reference with no problems.

let r2 = &mut s;
```

note:
1. This demonstrates what we mean by "at a time"
   1. Basically, the rules apply for any given scope

---

## Borrowing

File: `03-ownership\src\bin\02-borrowing.rs`

```rust []
// This block will compile
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{r3}");

// Will not compile if we add the following line
// println!("{r1} and {r2}");
```

note:
1. Compiler allows this as long as we don't use the immutable references after the mutable reference is created

---

## Structs

File: `04-structs\src\main.rs`

```rust []
struct MyStruct {};

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

struct Point2d (i32, i32);

```

note:
1. What is the size of `MyStruct`?
2. What is the size of `Person`?
   1. `String` is a heap-allocated type
      1. String is 3*word (pointer, length, capacity)
   2. `u8` is a stack-allocated type
      1. u8 is 1 byte but is padded to 1 word for alignment
   3. `Person` is 2*3*word + 1*word bytes
      1. 64bit system: 56 bytes NOT 49

---

## Structs

File: `04-structs\src\main.rs`

```rust []
// Instantiating a struct
let person = Person {
    first_name: String::from("John"),
    last_name: String::from("Doe"),
    age: 30,
};

let point = Point2d (0, 0);
```

note:
1. Usually, you would only do this in the constructor or for your own structs

---

## Structs

File: `04-structs\src\main.rs`

```rust []
// Access fields of the Person
println!("first_name = {}", person.first_name);
println!("last_name = {}", person.last_name);
println!("age = {}", person.age);

// Access fields of the Point2d
println!("x = {}", point.0);
println!("y = {}", point.1);
```

```rust []
// or like this
let Point2d (x, y) = point;
println!("x = {}, y = {}", x, y);
```

note:
1. Accessing tuple structs like this is not very readable
   1. Prefer to unpack the tuple into variables

---

## Structs

File: `04-structs\src\main.rs`

::: block <!-- element style="font-size: 0.85em; width: 27em" -->

```rust []
// Structs can have associated functions & methods
impl Person {
   // associated function
   fn new(first_name: &str, last_name: &str, age: u8) -> Self {
      Person {
         first_name: first_name.to_string(),
         last_name: last_name.to_string(),
         age,
      }
   }

   // method
   fn full_name(&self) -> String {
      format!("{} {}", self.first_name, self.last_name)
   }
}
```
:::
note:
This is where most of the object-oriented programming happens in Rust
1. `new` is the conventional name for a constructor
2. `full_name` is a method because it takes `&self` as an argument 
   1. What would happen if we took `self` instead of `&self`?

---

## Structs

File: `04-structs\src\main.rs`

```rust []
// Using associated functions & methods
let person = Person::new("Jane", "Doe", 25);
println!("full_name = {}", person.full_name());
```

note:
1. Associated functions are called with `::`
2. Methods are called with `.`