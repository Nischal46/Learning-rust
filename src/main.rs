#[derive(Debug)]
struct VehicleDetails {
    vehicle_type: String,
    brand: String,
    price: u32,
}

trait VehicleTAX {
    fn tax(&self) -> u32;
}

impl VehicleTAX for VehicleDetails {
    fn tax(&self) -> u32 {
        (&self.price) / 100 * 13
    }
}

fn main() {
    // use VehicleTAX;

    let vehicle1 = VehicleDetails {
        vehicle_type: "SUV".to_string(),
        brand: "Ford".to_string(),
        price: 20000000
    };

    println!("{:#?}", vehicle1);
    println!("{}", vehicle1.tax());
}
