
// Function with no return value
fn print_result(result: i32) /* -> () */ {
    println!("result is: {}", result);
}

// Function with return value
fn add_numbers(x: i64, y: i32) -> i32 {
    x + y
}

// Function with multiple return values
fn add_and_multiply(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}


fn main() {
    let x = 5;
    let y = 2;

    let result = add_numbers(x, y);
    print_result(result);

    // What happens if x and y are i64?
    // let z: i64 = 5;
    // let w: i64 = 2;

    // let result = add_numbers(z, w);
    // println!("result = {}", result);
}
