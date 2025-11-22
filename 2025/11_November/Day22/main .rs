#[derive(Debug)]
struct Device{
    type: String
}

trait Owner {
    fn details(&self) -> String;
}

impl Owner for Device {
    fn details(&self) -> String {
        let response = format!("Nischal got {}", &self.type);
        response
    }
}

fn main() {
    let dev = Device {
        type: "Laptop".to_string();
    };

    println!(dev.details());
}