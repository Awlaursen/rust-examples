// OWNERSHIP
fn main() {
    // Scopes are defined by blocks like this one or functions
    {                           // s is not valid here, it’s not yet declared
        let s = "hello";  // s is valid from this point forward
        
        // do stuff with s
        println!("{}", s);
    }                           // this scope is now over, and s is no longer valid
    // println!("{}", s); // This will not compile
    
    // Move of ownership from s1 to s2
    let s1 = String::from("hello");
    let s2 = s1;    // Move happens because String is a heap-allocated type
                            // and it is not a `Copy` type

    println!("{s1}, world!"); // This will not compile


    // Clone of s3 to s4
    let s3 = String::from("hello");
    let s4 = s3.clone();    // Clone is called explicitly to create a deep copy

    println!("{}, world!", s3); // This will compile

    
    // Ownership and functions
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
  // scope

a_string  // a_string is returned and moves out to the calling function
}