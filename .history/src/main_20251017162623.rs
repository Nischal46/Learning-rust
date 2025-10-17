mod closures;
fn main(){

    //Differences between functions and procedures

    //functions - that return value

    let fn_return_val = function_example(3,2);
    println!("Value getting from fn return, {}", fn_return_val);

    //procedure - same as function but it didnot return value

    procedure_example();

    //String vs string slice
    //String allocates memory in help
    //whereas str is little bit tricky and difficult as it can store memory on both heap as well as
    //stack

    let s1 = String::from("String allocated in heap");
    let s2 = &s1; //we had passed ownership to new variable

    println!("Printing value of String in reverse order as it supports by macro as Strings are -- {1}, --{0}", s1, s2);

    let s3: &str = "Hello";
    let s4 = s3; //this makes a new string slice in stack as its computation is cheap as compared to

    println!("Printing value of string slices and they are {} and {}", s3, s4);

    let product1 = Product::new("Laptop".to_string(), "Dell".to_string(), 68000);
    println!("Product1: {:#?}", product1);
    println!("{}", product1.details());

    closures::closure_fn_concept();

    //At first, all the declared variables are immutable we can replace by mut
}

fn function_example(num1: i8, num2: i8) -> i8 {
    num1 + num2
}

fn procedure_example(){
    println!("Hey I am procedure. but i didnot return value, but i can accept parameter as well");
}

#[derive(Debug)]
struct Product {
    title: String,
    brand: String,
    price: i64
}

impl Product {
    fn new(title: String, brand: String, price: i64) -> Self {
        Self {
            title,
            brand,
            price
        }
    }

    fn details(&self) -> String {
        let response = format!("You had picked {} laptop", self.brand);
        response
    }
}
