use std::thread;
use std::time::Duration;

fn expensive_fn_calculation() -> String {
    println!("It takes time. Plase wait....");
    thread::sleep(Duration::from_secs(3));
    format!("Wait for 3 sec and then finish......")
}

fn fn_pass_parameter_as_js(execute: fn() -> String) -> String {
    execute();
    let res = String::from("Value we recieved and we return as ");
    res
}

pub fn init() {
    println!("This is the concept of anonymous fn");

    let exp_fn_call = expensive_fn_calculation();
    println!("Returning response: {}", exp_fn_call);

    // NOTE: Here we had pass function pointer in function parameter
    let accept_string_return = fn_pass_parameter_as_js(expensive_fn_calculation);
    println!(
        "Logging of accept_string_return ----- {}",
        accept_string_return
    );
}
