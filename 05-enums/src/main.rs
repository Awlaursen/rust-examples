enum Shape {
    Circle(i32),
    Rectangle(i32, i32),
    Triangle(i32, i32),
}

fn describe_shape(shape: Shape) {
    // describe_shape takes a Shape as a parameter and uses match to print:

    // "Circle with radius X" for a Circle.
    // "Rectangle with width W and height H" for a Rectangle.
    // "Triangle with base B and height H" for a Triangle.

    // If describe_shape is called with any shape with both 
    // dimensions greater than 10, print an additional message, 
    // "Large shape detected."

    // Use a match statement here to handle each variant
    !todo!()
}

fn main() {
    let shapes = [
        Shape::Circle(5),
        Shape::Rectangle(8, 15),
        Shape::Triangle(12, 11),
    ];
    
    for shape in shapes {
        describe_shape(shape);
    }
}