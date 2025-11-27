//Rust trait concept

trait Describe {
    fn describe(&self) -> String;
}

#[derive(Debug)]
struct HeavyVehicle {
    brand: String,
    model: String
}

#[derive(Debug)]
struct LightVehhicle {
    brand: String,
    manufacurer: String
}

impl Describe for HeavyVehicle {
    fn describe(&self) -> String {
        let response = format!("Heavy vehicle introduced. Its {} and {}", self.brand, self.model);
        response
    }
}

impl Describe for LightVehhicle {
    fn describe(&self) -> String {
        let response = format!("Light vehicle introduced. Its {} and {}", self.brand, self.manufacurer);
        response
    }
}

fn main() {
    let obj1 = HeavyVehicle {
        brand: "TATA".to_string(),
        model: "1994-ER-RES".to_string()
    };

    let obj2 = LightVehhicle {
        brand: "pulsar-150".to_string(),
        manufacurer: "bajaj".to_string()
    };

    println!("Logging of the object {:#?}", obj1);
    println!("Logging of the light vehicle {:#?}", obj2);

    println!("{}", obj1.describe());
    println!("{}", obj2.describe());
}
