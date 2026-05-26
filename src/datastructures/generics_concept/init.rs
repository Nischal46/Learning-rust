struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
enum Expect_num<T> {
    Some(T),
    None,
}

pub fn init() {
    println!("This is the concept of generics");

    let integer_generics = Point { x: 1, y: 4 };
    let float_generics = Point { x: 1.5, y: 2.5 };

    println!(
        "Logging of the integer x: {}, y: {}",
        integer_generics.x, integer_generics.y
    );

    println!(
        "Logging of the float x: {}, y: {}",
        float_generics.x, float_generics.y
    );

    let guessing_num = Expect_num::Some(34);
    println!("Guessing the num: {:?}", guessing_num);

    let guess_float = Expect_num::Some(23.34);
    println!("Guessing the float: {:?}", guess_float);
}
