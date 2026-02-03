#[derive(Debug)]
enum Area {
    Circle(f64, f64),
    Rectangle(u8, u8),
}

impl Area {
    //associative fn and deal with Self
    fn circle_constructor(pi: f64, radius: f64) -> Self {
        Self::Circle(pi, radius)
    }

    fn rectangle_constructor(length: u8, breadth: u8) -> Self {
        Self::Rectangle(length, breadth)
    }

    //methods
    fn area(&self) -> f64 {
        // * refers for deferencing of the reference of the memory value
        match *self {
            Area::Circle(pi, radius) => pi * radius * radius,
            Area::Rectangle(length, breadth) => (2 * (length + breadth)).into(),
        }
    }
}

fn main() {
    let area_circle_1 = Area::circle_constructor(3.14, 7.0);
    println!("{:?}", area_circle_1);
    println!("Area of circle: {:?}", area_circle_1.area());

    let area_rectangle_1 = Area::rectangle_constructor(12, 10);
    println!("{:?}", area_rectangle_1);
    println!("Area of rectangle: {:?}", area_rectangle_1.area())
}
