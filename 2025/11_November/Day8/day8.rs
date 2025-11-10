
trait Greet {
    fn greet(&self) -> String;
}

#[derive(Debug)]
struct Customer{
    name: String,
    email: String
}

impl Greet for Customer {
    fn greet(&self) -> String {
        let greet = "hello".to_string();
        greet
    }
}

fn main() {
    let obj = Customer {
        name: "Nischal".to_string(),
        email: "nischal@rust.com".to_string()
    };

    println!("{:#?}", obj);
    println!("{}", obj.greet());

}