// traits with generics

trait Response {
    fn display_response(&self) -> String;
}

#[derive(Debug)]
struct API{
    message: String,
    data: String
}

impl Response for API {
    fn display_response(&self) -> String {
        let response = format!("Return message: {}", self.data);
        response
    }
}

fn notify_message<T: Response>(item: T) {
    println!("Hello hello! {}", item.display_response());
}

//other trait bound

fn print_input<T: std::fmt::Display>(value: T){
    println!("User had typed {} keyword", value);
}

fn main() {

    let obj = API {
        message: "Data fetched succesfully".to_string(),
        data: "Contains data".to_string()
    };

    notify_message(obj);

    print_input("Good Luck".to_string());


}