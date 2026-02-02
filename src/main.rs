#[derive(Debug)]
struct Citizen<'a> {
    name: &'a str,
    contact: u64,
    age: u8,
}

impl <'a> Citizen<'a> {
    fn new(name: &'a str, contact: u64, age: u8) -> Self {
        Self {
            name: name,
            contact: contact,
            age: age,
        }
    }

    fn check_voting_eligibility(&self){
        if *&self.age > 18 {
            println!("{} is eligible for voting.", &self.name);
        } 
        else {
            println!("{} is not eligible for voting.", &self.name);
        }
    }
}

fn main() {
    // struct and impl block concept

    let person1 = Citizen::new("nischal", 9809875678, 23);
    println!("Person 1 object details: {:?}", person1);
    person1.check_voting_eligibility();

    let person2 = Citizen::new("baniya", 9786543234, 17);   
    println!("Person2 object details: {:?}", person2);
    person2.check_voting_eligibility();
}
