// Enums & Pattern Matching in Rust

// Normal "c-style" enum
enum Direction {
    Up,     // 0
    Down,   // 1
    Left,   // 2
    Right,  // 3
}

// Enum with data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Method to process the Message enum
impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => println!("Changing color to ({}, {}, {})", r, g, b),
        }
    }
}

// Most common use of enums is to create Option<T> or Result<T, E>
// Result<T, E> is used for error handling
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        return Err("Cannot divide by zero".to_string());
    }
    Ok(x / y)
}

// Option<T> is used for optional values
fn get_first_element(list: Vec<i32>) -> Option<i32> {
    if list.is_empty() {
        return None;
    }
    Some(list[0])
}


fn main() {
    
    // C-style enum
    let dir = Direction::Up;

    // Match statement to process the enum
    match dir {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving down"),
        Direction::Left => println!("Moving left"),
        Direction::Right => println!("Moving right"),
    }


    // Enum with data and method to process the enum
    let action = Message::ChangeColor(255, 0, 255); // purple
    action.process();

    // Using Result<T, E> for error handling
    let result = divide(10.0, 0.0);
    match result {
        Ok(value) => println!("result = {}", value),
        Err(error) => println!("error = {}", error),
    }
    
    // Using unwrap() to get the value from Result<T, E>
    println!("result = {}", divide(10.0, 2.0).unwrap());

    // Using Option<T> for optional values
    let list = vec![1, 2, 3, 4, 5];
    let first = get_first_element(list);
    match first {
        Some(value) => println!("first = {}", value),
        None => println!("list is empty"),
    }

}
