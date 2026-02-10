#[derive(Debug)]
struct Developer<'a> {
    name: &'a str,
    postion: &'a str
}

fn main<'a>(){
    let dev_1 = Developer {
        name: "Nischal",
        postion: "Senior Rust Engineer"
    };

    let x = 5;

    let grap_value;

    {
        let var_a = &x;
        println!("Accessing value of a: {}", var_a);
    }

    {
        let make_lifetime: &'a i8 = 3;
        grap_value = &make_lifetime;
        println!("Inside scope lifetime: {}", make_lifetime);
    }

    println!("Logging of the object of developer: {:#?}", dev_1);
    println!("Accessing of lifetime from outside: {}", make_lifetime);
}
