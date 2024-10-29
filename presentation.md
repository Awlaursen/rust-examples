---
defaultTemplate: "[[tpl_satlab]]"
---
<style> :root {--r-code-font: "FiraCode Nerd Font";} .reveal .hljs {min-height: 50%;} </style>
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

