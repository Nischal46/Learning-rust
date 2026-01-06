
#[derive(Debug)]
struct Empoyee {
    employee_name: String,
    employee_email: String,
}

fn main() {

    let closure_concept = |x| x + 2;

    let emp1 = Empoyee {
        employee_name: "Nischal".to_string(),
        employee_email: "nischal@nisal.com".to_string(),
    };

    let num1 = closure_concept(3);

    println!("Employee: {:?}", emp1);
}