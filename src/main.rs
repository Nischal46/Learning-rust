mod module_concept;
fn main() {

    let mut vector_store_num:Vec<u8> = Vec::new();

    let mut employee_vector:Vec<module_concept::EmployeeDetails> = Vec::new();

    let mut input = String::new();
    println!("Enter some input: ");
    std::io::stdin().read_line(&mut input);

    println!("You had enter {}", input);

    match input.trim().parse::<i8>() {
        Ok(num) => println!("You had {}!", num),
        Err(_) => println!("You had entered an invalid number!"),
    }

    for n in 0..5 {
        let response = module_concept::check_for_ownership();
        vector_store_num.push(response + n);
    }

    println!("You had {:?}", vector_store_num);

    print!("-------------------------------------------------------------------------");
    println!("Details of employee");

    for n in 1..3 {
        let output = module_concept::gather_details();
        employee_vector.push(output);
    }

    println!("There are {} employees", employee_vector.len());
    println!("stored employees {:#?} ", employee_vector);
}