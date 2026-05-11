#[derive(Debug)]
enum Role {
    User,
    Admin,
    Superadmin,
}

#[derive(Debug)]
enum Shape {
    Rectangle { length: i32, breadth: i32 },
    Square { length: i32 },
}

pub fn concept1() {
    println!("This is from inside of other");
    let choosen_role = Role::Admin;

    println!("Choosen role is {:?}", choosen_role);

    let choosen_shape = Shape::Rectangle {
        length: 5,
        breadth: 4,
    };
    println!("Choosen shape is {:?}", choosen_shape);

    let mut userdetails1 = Userdetails {
        name: "nischal",
        email: "nischal@dev.com",
    };

    let tansferObj = greet(&mut userdetails1);

    println!(
        "Logging of the details object of the user: {:?}",
        userdetails1
    );
}

#[derive(Debug)]
struct Userdetails<'a> {
    name: &'a str,
    email: &'a str,
}

fn greet(obj: &mut Userdetails) {
    obj.email = "nischal.senior@dev.com";
    println!("We got here --- {:?}", obj)
}
