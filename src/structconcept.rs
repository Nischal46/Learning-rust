trait AreaTrait {
    fn area(&self) -> u8;
}

trait ResultTrait {
    fn result(&self) -> &str;
}

#[derive(Debug)]
struct Rectangle {
    length: u8,
    breadth: u8,
}

#[derive(Debug)]
struct Traingle {
    breadth: u8,
    height: u8,
}

//rust donot directly multiple trait

impl AreaTrait for Rectangle {
    fn area(&self) -> u8 {
        2 * (self.breadth + self.length)
    }
}

impl ResultTrait for Rectangle {
    fn result(&self) -> &str {
        let res = "result found";
        res
    }
}

impl AreaTrait for Traingle {
    fn area(&self) -> u8 {
        (self.breadth * self.height) / 2
    }
}

pub fn struct_concept() {
    let rec = Rectangle {
        length: 12,
        breadth: 15,
    };

    let trianle_1 = Traingle {
        breadth: 12,
        height: 10,
    };

    println!("Area of triangle: {} cm^2", trianle_1.area());

    println!("Result of area of rectangle: {} cm^2", rec.area());
}
