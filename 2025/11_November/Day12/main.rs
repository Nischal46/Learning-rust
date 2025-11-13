//NOTE: concept of anonymous function

fn test_closures() -> i8{
    let input = |x| x;
    let result = input(4);
    result
}


trait Brand {
    fn check_brand(&self) -> String;
}

#[derive(Debug)]
struct Product {
    title: String,
    brand: String
}

impl Brand for Product {
    fn check_brand(&self) -> String {
        let result = format!("Extract brand name: {}", &self.brand);
        result
    }
}

fn main() {

    let closure_fn_value = test_closures();
    println!("{}", closure_fn_value);

    let numbers = vec![12,23,34,45,56,67];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

    println!("{:?}", doubled);

    let product1 = Product {
        title: "Laptop".to_string(),
        brand: "DELL".to_string()
    };

    println!("{:#?}", product1);
    println!("{}", product1.check_brand());

    for n in 0..5 {
        println!("{}: loop", n);
    }

}