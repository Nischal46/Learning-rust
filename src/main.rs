#[derive(Debug)]
struct VehicleDetails {
    vehicle_type: String,
    brand: String,
    price: u32,
}

trait VehicleTAX {
    fn tax(&self) -> u32;
}

trait ScrapVehicleName {
    fn get_vehicle_name(&self) -> String;
}

impl VehicleTAX for VehicleDetails {
    fn tax(&self) -> u32 {
        (&self.price) / 100 * 13
    }
}

impl ScrapVehicleName for VehicleDetails {
    fn get_vehicle_name(&self) -> String {
        let result = format!("Vehicle name: {}", &self.vehicle_type);
        result
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
    println!("{}", vehicle1.get_vehicle_name());

}
