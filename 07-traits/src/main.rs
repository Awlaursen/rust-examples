// Define the Circle struct
struct Circle(f64);

// Implement methods for Circle
impl Circle {
    // Associated function to create a new Circle
    fn new(radius: f64) -> Self {
        todo!("Return a new instance of Circle with the given radius")
    }

    // Method to calculate area
    fn area(&self) -> f64 {
        todo!("Return the area (π * r^2)")
    }

    // Method to calculate circumference
    fn circumference(&self) -> f64 {
        todo!("Return the circumference (2 * π * r)")
    }
}

// Define the HasPerimeter trait
trait HasPerimeter {
    // Method signature for perimeter
    fn perimeter(&self) -> f64;
}

// Implement HasPerimeter for Circle
impl HasPerimeter for Circle {
    fn perimeter(&self) -> f64 {
        todo!("Implement using Circle's circumference method")
    }
}

fn main() {
    let circle = Circle::new(5.0);

    // Print area and perimeter
    println!("Area: {}", circle.area());
    println!("Perimeter: {}", circle.perimeter());
}
