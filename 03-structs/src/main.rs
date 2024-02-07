
// Structs are used to create custom data types
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

// Structs can have associated functions & methods
impl Person {
    // associated function
    fn new(first_name: &str, last_name: &str, age: u8) -> Person {
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
    

fn main() {
    // Create a new Person
    let person = Person::new("John", "Doe", 30);
    println!("person = {:?}", person);

    // Access fields of the Person
    println!("first_name = {}", person.first_name);
    println!("last_name = {}", person.last_name);
    println!("age = {}", person.age);

    // Call a method on the Person
    println!("full_name = {}", person.full_name());
}
