mod module_concept;
mod common_collection;

fn main() {

    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    for n in 0..5 {
        println!("Hello, {}", name);

    }


}