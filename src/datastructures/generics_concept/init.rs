//NOTE: using of Generics is to reduce code and make reusable
use std::{
    fmt::{Display, Formatter},
    ops::Add,
};
// to deal with int and float

//#[derive(Debug)]
struct Coordinates<T> {
    x: T,
    y: T,
}

// NOTE: we defined std::fmt::Display instead of Debug rait from derive
impl<T: Display> Display for Coordinates<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Add<Output = T> + Copy> Coordinates<T> {
    fn sum(&self) -> T {
        self.x + self.y
    }
}

impl<T: Display> Coordinates<T> {
    fn details(&self) {
        println!("only logging first coordinates: {}", self.x)
    }
}

pub fn init() {
    println!("---------------------Generics concept----------------------");
    let point_int = Coordinates { x: 2, y: 3 };

    println!("Printing of int Coordinates ----- {}", point_int);
    println!("Adding of the sum of point: {}", point_int.sum());

    let point_float = Coordinates { x: 1.2, y: 2.3 };
    println!("Printing of float coordinated ----- {}", point_float);
    println!("Printing the sum of float point: {}", point_float.sum());
    point_float.details();
}
