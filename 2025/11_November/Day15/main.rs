use std::collections::HashMap;

trait Area {
    fn area(&self) -> String;
}

#[derive(Debug)]
struct Rectangle {
    length: u8,
    breadth: u8,
}

#[derive(Debug)]
struct Triangle {
    breadth: u8,
    height: u8,
}

impl Area for Rectangle {
    fn area(&self) -> String {
        let area = &self.length * &self.breadth;
        let response = format!("Area of rectangle: {} cm^2", area);
        response
    }
}

impl Area for Triangle {
    fn area(&self) -> String {
        let area = (&self.breadth / 2) * &self.height;
        let response = format!("Area of triangle: {} cm^2", area);
        response
    }
}

fn main() {
    let rectangle1 = Rectangle {
        length: 3,
        breadth: 5,
    };

    let triangle1 = Triangle {
        breadth: 10,
        height: 7,
    };

    println!("Object of Rectangle: {:#?}", rectangle1);
    println!("Object of Triangle: {:#?}", triangle1);

    println!("{}", rectangle1.area());
    println!("{}", triangle1.area());

    println!("------------------------------------------------------");

    let mut product_collection = HashMap::new();

    product_collection.insert(
        "rust".to_string(),
        "i made iot embedded devices by this".to_string(),
    );

    product_collection.insert("js".to_string(), "language that teaches me programming".to_string());

    println!("Prinitng of the hash map: {:#?}", product_collection);

    let programming_lang = vec!["rust", "java", "js", "python"];

    for &lang in &programming_lang {
        match product_collection.get(lang) {
            Some(book) => println!("{lang} found in array"),
            None => println!("{lang} not found in array")
        }
    }
}
