// Define a struct with a lifetime parameter to store a reference
struct LongestStr<'a> {
    value: &'a str,
    length: usize,
}

// TODO: Add lifetime annotations to the function signature
fn choose_longest(a: &str, b: &str) -> LongestStr {
    todo!("Implement function logic to find the longest string")
}

fn main() {
    let s1 = String::from("Hello, world!");
    let s2 = "Rust";

    let result = choose_longest(&s1, s2);
    println!(
        "The longest string is: {}, with length: {}",
        result.value, result.length
    );
}
