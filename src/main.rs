mod module_concept;

fn main() {

    let mut input = String::new();
    println!("Enter some input: ");
    std::io::stdin().read_line(&mut input);

    println!("You had enter {}", input);

    match input.trim().parse::<i8>() {
        Ok(num) => println!("You had {}!", num),
        Err(_) => println!("You had entered an invalid number!"),
    }
}