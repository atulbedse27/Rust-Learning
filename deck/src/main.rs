trait Shape {
    fn area(&self) -> i32;
    fn perimeter(&self) -> i32;
}

struct Circle {
    radius: i32,
}

impl Circle {
    fn new(radius: i32) -> Self {
        Self { radius }
    }
}

struct Rectangle {
    length: i32,
    width: i32,
}

impl Rectangle {
    fn new(length: i32, width: i32) -> Self {
        Self { length, width }
    }
}

impl Shape for Circle {
    fn area(&self) -> i32 {
        3 * self.radius * self.radius
    }

    fn perimeter(&self) -> i32 {
        2 * 3 * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> i32 {
        self.length * self.width
    }

    fn perimeter(&self) -> i32 {
        2 * (self.length + self.width)
    }
}

// Generic (compile-time polymorphism)
fn calculate_total_area<T: Shape>(shape: &T) -> i32 {
    shape.area()
}

fn calculate_total_perimeter<T: Shape>(shape: &T) -> i32 {
    shape.perimeter()
}

fn main() {
    let circle = Circle::new(5);
    let rectangle = Rectangle::new(4, 6);

    println!("Area of circle: {}", calculate_total_area(&circle));
    println!("Area of rectangle: {}", calculate_total_area(&rectangle));
}