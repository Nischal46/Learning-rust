fn main(){

    //moving ownership to next function directly

    let s1 = 12;
    accept_ownership(s1);

    let s2 = 33;
    accept_only_reference(&s2);
    println!("The only way that we are able to get the value of s2 is because of referencing: {}", s2);

    let person_greet = greet("Nischal".to_string(), "nischal@gmail.com".to_string());
    println!("Displaying returning value from other fn, --- {}", person_greet);

    let rectangle_one: Rectangle = Rectangle {
        length: 10,
        breadth: 8
    };

    println!("Result of rectangle one: {:?}", rectangle_one);

    let rectangle_two: Rectangle = Rectangle {
        length: 8, 
        breadth: 6
    };

    println!("Result of rectangle two: {:?}", rectangle_two);

    area_calculate(rectangle_one);

    perimeter_rectangle(&rectangle_two);

    let emp1: Employee = Employee{
        name: "nischal".to_string(),
        email: "nischal@gmail.com".to_string(),
        experience: 2,
        role: "Full stack developer".to_string()
    };

    emp1.get_details();

    let traffic_light_status = TrafficLight::Green;

    match traffic_light_status {
        TrafficLight::Red=>println!("Stop vehicles. Red light"),
        TrafficLight::Green=>println!("Green Light. You can move"),
        TrafficLight::Yellow=>println!("Hold on.Yellow Light")
    }

}

fn accept_ownership(num: i32){
    println!("ownership moved from main to this sub function: {}", num);
}

fn accept_only_reference(num: &i32){
    println!("taking only reference: {}", num);
}

fn greet(name: String, email:String) -> String {
    let greet = format!("Welcome, Mr {} and your email is {}", name, email);
    greet
}

//Making complex data type Type

#[derive(Debug)] //this is trait
struct Rectangle {
    length: i16,
    breadth: i16
}

//NOTE: without object method manual function to calcute area and perimeter

// pass ownership
fn area_calculate(area: Rectangle){
    let result = area.length * area.breadth;
    println!("Area of rectangle ------- {}cm^2", result);
}

//pass reference
fn perimeter_rectangle(perimeter: &Rectangle){
    let result = (2 * perimeter.length) + (2 * perimeter.breadth);
    println!("Perimeter of rectangle ---------{}cm", result);
}

//Another way to implement method just like in just
//each object can associated with method as OOP in class or constructor function

#[derive(Debug)]
struct Employee {
    name: String,
    email: String,
    experience: u8,
    role: String
}

impl Employee {
    fn get_details(&self){
        println!("Hello {}. Your email: {}, and experience: {} yrs and your role {}", self.name, self.email, self.experience, self.role);
    }
}

//NOTE: Implementation of enum concept

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green
}
