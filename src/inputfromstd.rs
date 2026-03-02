use std;

pub fn take_input_from_user() {
    //to take input from user we have to go through std library

    let benchmark_number = 12;

    println!("Enter a number: ");
    let myinput = &mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();
    let final_input = myinput.replace("\n", "").parse::<u8>().unwrap();

    println!("You have type {}. and it is ", final_input);

    if final_input > benchmark_number {
        println!("Above than benchmark number");
    } else if final_input == benchmark_number {
        println!("Equal to benchmark number");
    } else {
        println!("Below than benchmark number");
    }
}
