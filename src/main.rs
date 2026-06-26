fn main() {
    println!("Main entry point app");

    let result: String;

    let output: &'static str;

    let answer: &str;

    {
        let inside_val = String::from("Hello hello");
        result = inside_val;

        output = "value generated from inside of scope";

        answer = "not static";
    }

    println!("Logging reult val: {}", result);
    println!("Logtging output val: {}", output);
    println!("Logging answer val: {}", answer);
}

fn check_static(inp: &mut str) -> &str {
    inp = "Hello form fn scope";
    return inp;
}
